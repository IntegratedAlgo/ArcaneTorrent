namespace TvdbApi;

public class TvDbList
{
    public int id { get; set; }
    public string name { get; set; }
    public string overview { get; set; }
    public string url { get; set; }
    public bool isOfficial { get; set; }
    public List<string> nameTranslations { get; set; }
    public List<string> overviewTranslations { get; set; }
    public List<Alias> aliases { get; set; }
    public int score { get; set; }
    public string image { get; set; }
    public bool imageIsFallback { get; set; }
    public object remoteIds { get; set; }
    public object tags { get; set; }
}