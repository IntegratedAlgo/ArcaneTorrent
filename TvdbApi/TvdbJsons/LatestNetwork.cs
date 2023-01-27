namespace TvdbApi.TvdbJsons;

public class LatestNetwork
{
    public int id { get; set; }
    public string name { get; set; }
    public string slug { get; set; }
    public List<string> nameTranslations { get; set; }
    public List<object> overviewTranslations { get; set; }
    public List<Alias> aliases { get; set; }
    public string country { get; set; }
    public int primaryCompanyType { get; set; }
    public object activeDate { get; set; }
    public object inactiveDate { get; set; }
    public CompanyType companyType { get; set; }
    public ParentCompany parentCompany { get; set; }
    public List<TagOption> tagOptions { get; set; }
}