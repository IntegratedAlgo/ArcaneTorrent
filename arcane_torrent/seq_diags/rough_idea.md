title CaptainJacksparrow

TVDB API -> RabbitMQ: QueryRequest(title, type, episode, season)
RabbitMQ -> ArcaneTorrent: Receives QueryRequest
ArcaneTorrent ->RabbitMQ: TorrentResult(Magnet)
RabbitMQ ->TVDB API: Receives Torrent Result
TVDB API ->RabbitMQ: DownloadRequest(Magnet)
RabbitMQ -> Downloader: Receives DownloadRequest
Downloader ->RabbitMQ: DownloadComplete(path)
RabbitMQ -> QualityChecker: Receives DownloadComplete
alt Quality Pass
QualityChecker ->RabbitMQ:PassedCheck
RabbitMQ ->TVDB API: Receives PassCheck
else Quality Fail
QualityChecker ->RabbitMQ: FailedCheck
RabbitMQ ->TVDB API: Receives FailedCheck
RabbitMQ ->Downloader: Receives FailedCheck
note over Downloader: Deletes Invalid File
end


