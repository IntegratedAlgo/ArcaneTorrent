using TvdbApi.TvdbJsons;

namespace TvdbApi;

public class ShowData
{
    public int id { get; set; }
    public string name { get; set; }
    public string slug { get; set; }
    public string image { get; set; }
    public List<string> nameTranslations { get; set; }
    public List<string> overviewTranslations { get; set; }
    public List<Alias> aliases { get; set; }
    public string firstAired { get; set; }
    public string lastAired { get; set; }
    public string nextAired { get; set; }
    public int score { get; set; }
    public Status status { get; set; }
    public string originalCountry { get; set; }
    public string originalLanguage { get; set; }
    public int defaultSeasonType { get; set; }
    public bool isOrderRandomized { get; set; }
    public string lastUpdated { get; set; }
    public int averageRuntime { get; set; }
    public List<Episode> episodes { get; set; }
    public string overview { get; set; }
    public string year { get; set; }
    public List<Artwork> artworks { get; set; }
    public OriginalNetwork originalNetwork { get; set; }
    public LatestNetwork latestNetwork { get; set; }
    public List<Genre> genres { get; set; }
    public List<Trailer> trailers { get; set; }
    public List<TvDbList> lists { get; set; }
    public List<RemoteId> remoteIds { get; set; }
    public List<Character> characters { get; set; }
    public AirsDays airsDays { get; set; }
    public string airsTime { get; set; }
    public List<Season> seasons { get; set; }
    public object tags { get; set; }
    public List<ContentRating> contentRatings { get; set; }
    public List<SeasonType> seasonTypes { get; set; }
}