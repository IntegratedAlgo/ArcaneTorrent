# TVDB API Gen
url: https://thetvdb.github.io/v4-api/swagger.yml

docker run --rm -v "/home/codingjinxx/Git/arcane_torrent_rewrite/arcane_torrent/tvdb_client:/local" openapitools/openapi-generator-cli generate \
    -i https://thetvdb.github.io/v4-api/swagger.yml \
    -g rust \
    -o /local

# Neo4j