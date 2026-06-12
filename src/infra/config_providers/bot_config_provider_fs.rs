use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
};

use crate::types::{
    traits::BotConfigProvider,
    values::{BotConfig, BotKind, BotName},
};

#[derive(Debug, Default)]
pub struct BotConfigProviderFs {
    pub dir_path: String,
    all_configs: HashMap<String, BotConfig>,
}

impl BotConfigProviderFs {
    pub fn new(dir_path: &str) -> Self {
        Self {
            dir_path: dir_path.to_string(),
            ..Default::default()
        }
    }

    async fn read_config(entry: &DirEntry) -> Result<(BotName, BotConfig)> {
        async {
            let path = entry.path();
            if path.is_dir() {
                Err(anyhow!("not a file: {path:?}"))?
            }
            let file_content = fs::read_to_string(path)?;
            let cfg = match Self::bot_kind(entry)? {
                BotKind::Single => BotConfig::Single(serde_json::from_str(&file_content)?),
                BotKind::Multi => BotConfig::Multi(serde_json::from_str(&file_content)?),
            };
            let bot_name = Self::bot_name(entry)?;
            Ok::<(String, BotConfig), anyhow::Error>((bot_name, cfg))
        }
        .await
        .with_context(|| format!("read_config for {:?}", entry.path()))
    }

    fn bot_kind(entry: &DirEntry) -> Result<BotKind> {
        let file_name = entry.file_name();
        let ctx = format!("bot_kind_from_fname: {file_name:?}");
        let file_name = file_name.to_str().with_context(|| ctx.clone())?;
        if file_name.contains("single") {
            Ok(BotKind::Single)
        } else if file_name.contains("multi") {
            Ok(BotKind::Multi)
        } else {
            Err(anyhow!("unknown bot kind")).with_context(|| ctx)?
        }
    }

    fn bot_name(entry: &DirEntry) -> Result<BotName> {
        let path = entry.path();
        let ctx = format!("bot_name_by_file: {:?}", &path);
        let file_stem = path
            .file_stem()
            .with_context(|| ctx.clone())?
            .to_str()
            .with_context(|| ctx.clone())?;
        Ok(file_stem.to_string())
    }
}

#[async_trait]
impl BotConfigProvider for BotConfigProviderFs {
    // Читаем все конфиги и сохраняем в словарь all_configs.
    // Ошибка только если проблема с директорией.
    // Если ошибка при чтении или парсинге файлом, то просто логирование
    async fn read_configs(&mut self) -> Result<()> {
        async {
            let ctx = format!("read_configs: {}", self.dir_path);
            log::debug!("=== FS READ CONFIGS ===");
            for entry in fs::read_dir(&self.dir_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }
                // Eсли не можем прочитать конфиг бота,
                // то только логируем ошибку.
                // Это позволит при последующих обновлениях конфигов не падать на пустом месте
                match Self::read_config(&entry).await {
                    Ok((bot_name, cfg)) => {
                        self.all_configs.insert(bot_name, cfg);
                    }
                    Err(e) => log::error!("{ctx}: {e:#?}"),
                }
            }
            Ok::<(), anyhow::Error>(())
        }
        .await
        .with_context(|| format!("read_configs: {}", self.dir_path))
    }

    async fn get_config(&self, bot_name: &BotName) -> Result<Option<BotConfig>> {
        Ok(self.all_configs.get(bot_name).cloned())
    }
}
