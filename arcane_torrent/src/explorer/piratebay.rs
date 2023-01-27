use std::collections::HashMap;

use crate::model::Magnet;

use super::{Explorer, Result, QueryResolver, MediaType, Media, Query};
use async_trait::async_trait;
use reqwest::Client;
use thirtyfour::prelude::*;
use futures::executor::block_on;
use std::mem;



pub struct PirateBayExplorer {
    url: String,
    client: Client,
    driver: Option<Box<WebDriver>>
}

#[async_trait]
impl Explorer for PirateBayExplorer {
    async fn new(url: String) -> Self {
        let client = Client::new();
        let caps = DesiredCapabilities::chrome();
        let driver = Some(Box::new(WebDriver::new("http://localhost:9515", caps).await.expect("Failed to create webdriver")));

        PirateBayExplorer {
            url,
            client,
            driver
        }
    }

    async fn search(&self, query: Query, page_depth: u32) -> Result<Vec<Media>> {
        // Fetch using reqwest and parse using scraper 
        // to get magnet link
        let mut media: Vec<Media> = Vec::new();
        for page in 0..page_depth{
            let host = PirateBayExplorer::get_resolver()[&self.url.as_str()].resolve(&query, page);
            self.driver.as_ref().unwrap().goto(&host).await?;

            let text = self.driver.as_ref().unwrap().find_all(By::Css("#st")).await?;

            // Get all link hrefs
            let mut hrefs : Vec<String> = Vec::new();
            for link in text {
                let category = link.find(By::Css("#st > span.list-item.item-type > a:nth-child(1)")).await?.text().await?;
                if category.contains("Video") {
                    let href = link.find(By::Css("#st > span.list-item.item-name.item-title > a")).await?.attr("href").await?.unwrap();
                    hrefs.push(href);
                }
            }

            
            for href in hrefs {
                let cut = &href[1..href.len()];
                self.driver.as_ref().unwrap().goto(format!("{}{}", self.url, cut)).await?;
                let page = self.driver.as_ref().unwrap().find(By::Css("#description_container")).await;
                let page = match page {
                    Ok(page) => page,
                    Err(_) => continue
                };
                let title =  PirateBayExplorer::parse_element(page.find(By::Css("#name")).await).await;
                let size = PirateBayExplorer::parse_element(page.find(By::Css("#size")).await).await;
                let description = PirateBayExplorer::parse_element(page.find(By::Css("#description_text")).await).await;
                let seeders = PirateBayExplorer::parse_element(page.find(By::Css("#s")).await).await;
                let leechers = PirateBayExplorer::parse_element(page.find(By::Css("#l")).await).await;
                let category = PirateBayExplorer::parse_element(page.find(By::Css("#cat")).await).await;
                let magnet = { 
                    let temp = page.find(By::Css("#d > a")).await;

                    match temp {
                        Ok(temp) => match temp.attr("href").await {
                            Ok(temp) => temp.unwrap_or("".to_owned()),
                            Err(_) => "".to_owned()
                        },
                        Err(_) => "".to_owned()
                    }
                };
                
                if category.contains("TV") {
                    media.push(Media::TV {
                        title,
                        size,
                        description,
                        magnet: Magnet(magnet),
                        seeders: seeders.parse().unwrap(),
                        leechers: leechers.parse().unwrap(),
                        season: "".to_owned(),
                        episode: "".to_owned()
                    })
                }
                else if category.contains("Movie") {
                    media.push(Media::Movie {
                        title,
                        size,
                        description,
                        magnet: Magnet(magnet),
                        seeders: seeders.parse().unwrap(),
                        leechers: leechers.parse().unwrap(),
                    })
                }
            }
        }
        
        Ok(media)
    }
}

impl Default for PirateBayExplorer {
    fn default() -> Self {
        let url = "https://ukpiratebay.org/".to_owned();
        let client = Client::new();
        let caps = DesiredCapabilities::chrome();
        let mut driver : Option<Box<WebDriver>> = None;


        let async_fn = async {
            let _driver = Box::new(WebDriver::new("http://localhost:9515", caps).await.expect("Failed to create webdriver"));
            driver = Some(_driver);
        };
        block_on(async_fn);
        
        let driver = Some(driver.unwrap());

        PirateBayExplorer {
            url,
            client,
            driver
        }
    }
}

impl Drop for PirateBayExplorer {
    fn drop(&mut self) {
        let async_fn = async {
            let explorer = mem::replace(&mut self.driver, None);
            explorer.unwrap().quit().await.expect("Failed to quit webdriver");
        };
        block_on(async_fn);
    }
}

impl PirateBayExplorer {
    fn get_resolver() -> HashMap<&'static str, Box<dyn QueryResolver + 'static>> {
        let resolvers : HashMap<&'static str, Box<dyn QueryResolver + 'static>> = HashMap::from([
            ("https://ukpiratebay.org/", Box::new(UkPirateBayOrg::new()) as Box<dyn QueryResolver + 'static>)
        ]);

        resolvers
    }

    async fn parse_element(elem: WebDriverResult<WebElement>) -> String {
        match elem {
            Ok(elem) => elem.text().await.unwrap(),
            Err(_) => String::from(""),
        }
    }
}

struct UkPirateBayOrg(&'static str);

impl UkPirateBayOrg {
    fn category_to_id(category: &MediaType) -> u8 {
        match category {
            MediaType::Movie => 201,
            MediaType::TV => 205,
        }
    }

    fn new() -> Self {
        UkPirateBayOrg("https://ukpiratebay.org/")
    }
}

impl QueryResolver for UkPirateBayOrg {
    fn resolve(&self, query: &Query, page: u32) -> String {
        // https://ukpiratebay.org/search.php?q=Lost&cat=201 

        let url = format!("{}search.php?q={}&cat={}&page={}",self.0, query.title, UkPirateBayOrg::category_to_id(&query.media_type), page);

        url
    }
}