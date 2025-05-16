use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.configure_sets(
        Update,
        (
            AppSystems::GameplayLogic,
            AppSystems::Despawn,
            AppSystems::UpdateUi,
        )
            .chain(),
    );
    app.configure_sets(
        Update,
        (
            GameplayLogicSystems::CalculateScores,
            GameplayLogicSystems::AdvanceDialogue,
            GameplayLogicSystems::ShootBadGuy,
        )
            .chain()
            .in_set(AppSystems::GameplayLogic),
    );
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    GameplayLogic,
    Despawn,
    UpdateUi,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum GameplayLogicSystems {
    CalculateScores,
    AdvanceDialogue,
    ShootBadGuy,
}

/*
But just be aware that PostUpdate is a biiiit easy to mess up
There are some rendering systems there that assume you're doing things ordered relative to it
I think despawning entities is fine
But I'm not entirely sure
Do not inserting entities, however. That will crash every now and then due to scheduling ambiguities.

*/
