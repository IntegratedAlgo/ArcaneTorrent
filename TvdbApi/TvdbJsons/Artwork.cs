namespace TvdbApi;

public class Artwork
{
    public int id { get; set; }
    public string image { get; set; }
    public string thumbnail { get; set; }
    public string language { get; set; }
    public int type { get; set; }
    public int score { get; set; }
    public int width { get; set; }
    public int height { get; set; }
    public int thumbnailWidth { get; set; }
    public int thumbnailHeight { get; set; }
    public int updatedAt { get; set; }
    public Status status { get; set; }
    public object tagOptions { get; set; }
}