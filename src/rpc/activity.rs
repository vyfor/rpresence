#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActivityAssets<'a> {
    pub(crate) large_image: Option<&'a str>,
    pub(crate) large_text: Option<&'a str>,
    pub(crate) small_image: Option<&'a str>,
    pub(crate) small_text: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActivityButton<'a> {
    pub(crate) label: &'a str,
    pub(crate) url: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Activity<'a> {
    pub(crate) ty: ActivityType,
    pub(crate) details: Option<&'a str>,
    pub(crate) state: Option<&'a str>,
    pub(crate) assets: Option<ActivityAssets<'a>>,
    pub(crate) buttons: Option<&'a [ActivityButton<'a>]>,
    pub(crate) timestamps: Option<ActivityTimestamps>,
    pub(crate) party: Option<ActivityParty<'a>>,
    pub(crate) secrets: Option<ActivitySecrets<'a>>,
    pub(crate) instance: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ActivityType {
    #[default]
    Playing,
    Listening,
    Watching,
}

impl ActivityType {
    pub fn to_u8(&self) -> u8 {
        match self {
            ActivityType::Playing => 0,
            ActivityType::Listening => 2,
            ActivityType::Watching => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActivityTimestamps {
    pub(crate) start: Option<u128>,
    pub(crate) end: Option<u128>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActivityParty<'a> {
    pub(crate) id: Option<&'a str>,
    pub(crate) size: Option<[u8; 2]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActivitySecrets<'a> {
    pub(crate) join: Option<&'a str>,
    pub(crate) spectate: Option<&'a str>,
    pub(crate) match_id: Option<&'a str>,
}

impl<'a> Activity<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn details(mut self, details: &'a str) -> Self {
        self.details = Some(details);
        self
    }

    pub fn state(mut self, state: &'a str) -> Self {
        self.state = Some(state);
        self
    }

    pub fn assets(mut self, assets: ActivityAssets<'a>) -> Self {
        self.assets = Some(assets);
        self
    }

    pub fn buttons(mut self, buttons: &'a [ActivityButton<'a>]) -> Self {
        self.buttons = Some(buttons);
        self
    }

    pub fn timestamps(mut self, timestamps: ActivityTimestamps) -> Self {
        self.timestamps = Some(timestamps);
        self
    }

    pub fn start_time(mut self, time: u128) -> Self {
        self.timestamps
            .get_or_insert_with(ActivityTimestamps::default)
            .start = Some(time);
        self
    }

    pub fn end_time(mut self, time: u128) -> Self {
        self.timestamps
            .get_or_insert_with(ActivityTimestamps::default)
            .end = Some(time);
        self
    }

    pub fn party(mut self, party: ActivityParty<'a>) -> Self {
        self.party = Some(party);
        self
    }

    pub fn party_id(mut self, id: &'a str) -> Self {
        self.party.get_or_insert_with(ActivityParty::default).id = Some(id);
        self
    }

    pub fn party_size(mut self, size: [u8; 2]) -> Self {
        self.party.get_or_insert_with(ActivityParty::default).size = Some(size);
        self
    }

    pub fn secrets(mut self, secrets: ActivitySecrets<'a>) -> Self {
        self.secrets = Some(secrets);
        self
    }

    pub fn join_secret(mut self, secret: &'a str) -> Self {
        self.secrets
            .get_or_insert_with(ActivitySecrets::default)
            .join = Some(secret);
        self
    }

    pub fn spectate_secret(mut self, secret: &'a str) -> Self {
        self.secrets
            .get_or_insert_with(ActivitySecrets::default)
            .spectate = Some(secret);
        self
    }

    pub fn match_secret(mut self, secret: &'a str) -> Self {
        self.secrets
            .get_or_insert_with(ActivitySecrets::default)
            .match_id = Some(secret);
        self
    }

    pub fn instance(mut self, instance: bool) -> Self {
        self.instance = Some(instance);
        self
    }

    pub fn ty(mut self, ty: ActivityType) -> Self {
        self.ty = ty;
        self
    }

    pub fn large_image(mut self, large_image: &'a str) -> Self {
        self.assets
            .get_or_insert_with(ActivityAssets::default)
            .large_image = Some(large_image);
        self
    }

    pub fn large_text(mut self, large_text: &'a str) -> Self {
        self.assets
            .get_or_insert_with(ActivityAssets::default)
            .large_text = Some(large_text);
        self
    }

    pub fn small_image(mut self, small_image: &'a str) -> Self {
        self.assets
            .get_or_insert_with(ActivityAssets::default)
            .small_image = Some(small_image);
        self
    }

    pub fn small_text(mut self, small_text: &'a str) -> Self {
        self.assets
            .get_or_insert_with(ActivityAssets::default)
            .small_text = Some(small_text);
        self
    }
}

impl<'a> ActivityAssets<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn large_image(mut self, large_image: &'a str) -> Self {
        self.large_image = Some(large_image);
        self
    }

    pub fn large_text(mut self, large_text: &'a str) -> Self {
        self.large_text = Some(large_text);
        self
    }

    pub fn small_image(mut self, small_image: &'a str) -> Self {
        self.small_image = Some(small_image);
        self
    }

    pub fn small_text(mut self, small_text: &'a str) -> Self {
        self.small_text = Some(small_text);
        self
    }
}

impl<'a> ActivityButton<'a> {
    pub fn new(label: &'a str, url: &'a str) -> Self {
        Self { label, url }
    }
}

impl<'a> ActivityParty<'a> {
    pub fn new(id: &'a str, size: [u8; 2]) -> Self {
        Self {
            id: Some(id),
            size: Some(size),
        }
    }
}

impl<'a> ActivitySecrets<'a> {
    pub fn new(join: &'a str, spectate: &'a str, match_id: &'a str) -> Self {
        Self {
            join: Some(join),
            spectate: Some(spectate),
            match_id: Some(match_id),
        }
    }
}

impl ActivityTimestamps {
    pub fn with_start(start: u128) -> Self {
        Self {
            start: Some(start),
            end: None,
        }
    }

    pub fn with_end(end: u128) -> Self {
        Self {
            start: None,
            end: Some(end),
        }
    }
}
