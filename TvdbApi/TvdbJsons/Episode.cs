namespace TvdbApi;

public class Episode
{
    public int id { get; set; }
    public int seriesId { get; set; }
    public string name { get; set; }
    public string aired { get; set; }
    public int runtime { get; set; }
    public object nameTranslations { get; set; }
    public string overview { get; set; }
    public object overviewTranslations { get; set; }
    public string image { get; set; }
    public int? imageType { get; set; }
    public int isMovie { get; set; }
    public object seasons { get; set; }
    public int number { get; set; }
    public int seasonNumber { get; set; }
    public string lastUpdated { get; set; }
    public string finaleType { get; set; }
    public int airsBeforeSeason { get; set; }
    public int airsBeforeEpisode { get; set; }
    public string year { get; set; }
    public int? airsAfterSeason { get; set; }
}