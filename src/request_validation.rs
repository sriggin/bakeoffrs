use std::error::Error;
use std::path::Path;
use catboost2::Model;
use deadpool::managed::{Manager, RecycleResult};
use async_trait::async_trait;
use deadpool::managed;

type Pool = managed::Pool<ModelManager>;

pub struct RequestValidator {
    model: Pool,
}

struct ModelManager {
    path: &'static Path
}

struct SafeModel(Model);

impl SafeModel {
    fn get(&self) -> &Model {
        let SafeModel(m) = self;
        m
    }
}

//wow
unsafe impl Send for SafeModel {}

#[async_trait]
impl Manager for ModelManager {
    type Type = SafeModel;
    type Error = &'static str;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Model::load(self.path)
            .map(SafeModel)
            .map_err(|_| "failed to load model")
    }

    async fn recycle(&self, _obj: &mut Self::Type) -> RecycleResult<Self::Error> {
        Ok(())
    }
}

impl RequestValidator {
    pub fn load(path: &'static Path) -> Result<RequestValidator, Box<dyn Error>> {
        let manager = ModelManager { path };
        let pool = Pool::builder(manager).build()
            .map_err(|e| e.to_string())?;
        Ok(RequestValidator { model: pool })
    }

    pub async fn is_valid(&self, user_agent: &str, ip: &str) -> Result<bool, Box<dyn Error>> {
        let cat_features = vec![vec![user_agent.to_string(), ip.to_string()]];
        let safe_model = self.model.get().await.unwrap();
        let model = safe_model.get();
        let prediction = model.calc_model_prediction(vec![vec![]], cat_features)?;
        assert_eq!(1, prediction.len());

        Ok(prediction[0] > 0.5) // // bad and wrong and i don't care
    }
}
