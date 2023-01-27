#[cfg(test)]
pub mod bert_experiments {
    use arcane_torrent::information_extractor::InformationExtractor;
    use rust_bert::pipelines::question_answering::{QuestionAnsweringModel, QaInput};
    use serde::{Deserialize, Serialize};


    #[test]
    fn test_simple_question_answering() {
        let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
                                                        
        let question = String::from("Where does Amy live ?");
        let context = String::from("Amy lives in Amsterdam");
    
        let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);
        
        println!("{:?}", answers);
    }

    #[test]
    fn test_extracting_show_info() {
        let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
                                                        
        let question1 = String::from("When was the movie released");
        let question2 = String::from("What quality is the Movie");
        let question3 = String::from("What is the title of the movie");
        let context = String::from("Tenet (2020)  (1080p BluRay x265 HEVC 10bit EAC3)");
    
        let answers = qa_model.predict(&[QaInput { question: question1, context: context.clone() }, QaInput { question:  question2, context: context.clone() }, QaInput { question: question3, context: context.clone() }], 1, 32);
        
        println!("{:?}", answers);
    }


    #[test]
    fn test_information_extraction() {
        let extractor = InformationExtractor::new(0.5);
        
        let input = ShowDataTest {
            title: "Lucifer.S06E01.720p.WEB.H264-CAKES[ettv]".to_owned(),
            description: r#"
            ettv - ettv - Tv Shows
            
            -----------------------------------------------------
            https://www.ettvcentral.com/home/
            
            Lucifer.S06E01.720p.WEB.H264-CAKES
                                
            [Video Bitrate]:.............: 2369kb/s
            [Duration]:.............: 48 min 25 s
            [Video Codec]:.............: AVC
            [Resolution]:.............: 1280x720 px
            [Aspect Ratio]:.............: 16:9
            [Audio Codec]:.............: E-AC-3
            [Audio Bitrate]:.............: 640kb/s
            [Audio Channels]:.............: 5.1ch
            [Audio Language]:.............: English
                                
            Genre .......................: 
                                
            #ettv -> To avoid fakes, ALWAYS check that the torrent was added by ettv 
            
            
            To keep us going, support us by sharing for at least 24 hours or 1:1 ratio.
            
            Screens...
            
            https://snoopimages.com/image/elVBD
            
            https://snoopimages.com/image/elcY3
            
            https://snoopimages.com/image/elhKh
              
            https://snoopimages.com/image/elj9r
             
            https://snoopimages.com/image/elsTE"#.to_owned().replace(['[', ']'], "")
        };
        let result = extractor.extract_episode_and_season(&input.title).expect("Unexpected");
        println!("{:?}", result);
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    struct ShowDataTest {
        title: String,
        description: String
    }


}