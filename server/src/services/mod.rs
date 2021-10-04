mod github;

use self::github::GitHub;

pub struct Services {
    pub github: GitHub,
}

impl Services {
    pub fn new() -> Self {
        Services {
            github: GitHub::new(),
        }
    }
}
