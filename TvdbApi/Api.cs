using System.Net;
using System.Net.Http.Headers;
using System.Text;
using Newtonsoft.Json;

namespace TvdbApi;

/// <summary>
/// Api fot the TVDB
/// </summary>
public class Api
{
    public string Token { get; private set; } = null;
    private readonly string _apiKey;
    private readonly string _baseUrl = "https://api4.thetvdb.com/v4";

    /// <summary>
    /// Log into the TVDB and get an api token.
    /// </summary>
    /// <returns></returns>
    public bool Login()
    {
        try
        {
            var url = $"{this._baseUrl}/login";
            var client = new HttpClient();
            client.DefaultRequestHeaders
                .Accept
                .Add(new MediaTypeWithQualityHeaderValue("application/json"));

            client.DefaultRequestHeaders.Add(HttpRequestHeader.UserAgent.ToString(), "myrequester");

            var json = $"{{\"apikey\":\"{this._apiKey}\"}}";
            var body = new StringContent(json, Encoding.UTF8, "application/json");
            var response = client.PostAsync(url, body).Result;

            var status = response.StatusCode;

            if (status == HttpStatusCode.OK)
            {
                var r = response.Content.ReadAsStringAsync().Result;
                var j = JsonConvert.DeserializeObject<Root<LoginData>>(r);
                this.Token = j.Data.token;
                return true;
            }
            else return false;
        }
        catch (Exception e)
        {
            throw e;
        }
    }

    /// <summary>
    /// Gets show data for a the given showId (TVDB ID).
    /// </summary>
    /// <param name="showId"></param>
    /// <returns></returns>
    public ShowData? GetShowData(string showId)
    {
        var url = $"{this._baseUrl}/series/{showId}/extended?meta=episodes";

        try
        {
            var response = this.Request(url);
            var status = response.StatusCode;
            if (status == HttpStatusCode.OK)
            {
                var r = response.Content.ReadAsStringAsync().Result;
                var j = JsonConvert.DeserializeObject<Root<ShowData>>(r);
                var showData = j.Data;
                return showData;
            }
            else return null;
        }
        catch (Exception e)
        {
            throw e;
        }
    }

    /// <summary>
    /// Gets the show ids based on show name, will return multiple results
    /// </summary>
    /// <param name="showName"></param>
    /// <returns></returns>
    public List<ShowId>? GetShowId(string showName)
    {
        var url = $"{this._baseUrl}/search?query={showName}&type=series";

        try
        {
            var response = this.Request(url);
            var status = response.StatusCode;
            if (status == HttpStatusCode.OK)
            {
                var r = response.Content.ReadAsStringAsync().Result;
                var j = JsonConvert.DeserializeObject<Root<List<ShowId>>>(r);
                var showData = j.Data;
                return showData;
            }
            else return null;
        }
        catch (Exception e)
        {
            throw e;
        }
        return null;
    }

    private HttpResponseMessage Request(string fullUrl)
    {
        if (this.Token == null) this.Login();

        try
        {
            var client = new HttpClient();
            client.DefaultRequestHeaders
                .Accept
                .Add(new MediaTypeWithQualityHeaderValue("application/json"));
            client.DefaultRequestHeaders.Add("Authorization", $"Bearer {this.Token}");

            var response = client.GetAsync(fullUrl).Result;

            return response;

        }
        catch (Exception e)
        {
            throw e;
        }
    }

    public Api(string apiKey)
    {
        this._apiKey = apiKey;
    }
}