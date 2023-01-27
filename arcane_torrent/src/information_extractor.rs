use rust_bert::pipelines::question_answering::{QuestionAnsweringModel, QaInput};

type Result<T> = std::result::Result<T, PredictionError>;

#[derive(Debug, Clone)]
pub struct PredictionError;

pub struct InformationExtractor {
    pub threshhold: f32,
    model: QuestionAnsweringModel
}

#[derive(Debug)]
pub struct PredictionResult<T> {
    data: T,
    confidence: f64
}

#[derive(Debug)]
pub struct EpisodeSeasonPrediction {
    episode: PredictionResult<String>,
    season: PredictionResult<String>
}


impl InformationExtractor {
    pub fn extract_episode_and_season(&self, context: &String) -> Result<EpisodeSeasonPrediction> {
        let result = self.model.predict(&[
            QaInput{ question: "What episode?".to_owned(), context: context.clone() },
            QaInput{ question: "What season?".to_owned(), context: context.clone() }
        ], 1, 64);


        Ok(
            EpisodeSeasonPrediction { 
                episode: PredictionResult { 
                    data: result[0][0].answer.to_string(), 
                    confidence: result[0][0].score 
                }, 
                season: PredictionResult { 
                    data: result[1][0].answer.to_string(), 
                    confidence: result[1][0].score
                } }
        )
    }

    pub fn new(threshhold: f32) -> Self {
        InformationExtractor { 
            threshhold, 
            model: QuestionAnsweringModel::new(Default::default()).unwrap()
        }
    }
}