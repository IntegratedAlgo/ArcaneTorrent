namespace TvdbApi;

public class ContentRating
{
    public int id { get; set; }
    public string name { get; set; }
    public string country { get; set; }
    public string description { get; set; }
    public string contentType { get; set; }
    public int order { get; set; }
    public object fullname { get; set; }
}