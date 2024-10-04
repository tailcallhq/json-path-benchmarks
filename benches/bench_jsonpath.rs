use criterion::Criterion;
use jsonpath::Selector;
use serde_json::Value;

pub fn bench(c: &mut Criterion) {
    let json = std::fs::read_to_string("benches/data.json").unwrap();
    let json: Value = serde_json::from_str(json.as_str()).unwrap();
    let selector = Selector::new("$.sand.now.seeing.pound.method.motor.breathing.action.huge.relationship.fun.grade.express.music.which.everyone.fix.white.attached.break.bad.electricity.tail.title.prize.itself.completely.stared.lunch.particularly.plates.held.silver.seen.trail.cage.empty.from.sleep.lake.control.grain.changing.whispered.home.drove.add.widely.current.rubbed.whale.beneath.trail.explore.foot.salt.brought.drink.worth.dozen.solution.copper.moving.swing.neighbor.your.suit.were.harbor.forest.strike.favorite.start.concerned.draw.smallest.single.statement.favorite.stood.mill.freedom.basis.carry.naturally.parts.out.major.visit.appearance.sit.becoming.party.object.its.track.space.facing.experience").unwrap();

    c.bench_function("bench_jsonpath", |b| {
        b.iter(|| {
            let _titles: Vec<bool> = selector.find(&json).map(|t| t.as_bool().unwrap()).collect();
            // assert_eq!(titles.len(), 15840);
        })
    });
}
