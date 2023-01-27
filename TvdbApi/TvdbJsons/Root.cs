namespace TvdbApi;

public class Root<TData> where TData : class
{
    public string Status { get; set; }
    public TData Data { get; set; }
}