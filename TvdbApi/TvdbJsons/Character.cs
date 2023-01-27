namespace TvdbApi;

public class Character
{
    public int id { get; set; }
    public string name { get; set; }
    public int peopleId { get; set; }
    public int seriesId { get; set; }
    public object series { get; set; }
    public object movie { get; set; }
    public object movieId { get; set; }
    public object episodeId { get; set; }
    public int type { get; set; }
    public string image { get; set; }
    public int sort { get; set; }
    public bool isFeatured { get; set; }
    public string url { get; set; }
    public object nameTranslations { get; set; }
    public object overviewTranslations { get; set; }
    public object aliases { get; set; }
    public string peopleType { get; set; }
    public string personName { get; set; }
    public object tagOptions { get; set; }
    public string personImgURL { get; set; }
}