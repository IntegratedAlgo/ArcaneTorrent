namespace TvdbApi;

public class Season
{
    public int id { get; set; }
    public int seriesId { get; set; }
    public Type type { get; set; }
    public int number { get; set; }
    public List<object> nameTranslations { get; set; }
    public List<object> overviewTranslations { get; set; }
    public string image { get; set; }
    public int imageType { get; set; }
    public Companies companies { get; set; }
    public string lastUpdated { get; set; }
}