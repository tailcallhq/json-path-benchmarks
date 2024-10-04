use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::Value;
use serde_json_path::JsonPath;

fn create_json() -> Value {
    let json = include_str!("./data.json");

    serde_json::from_str(json).unwrap()
}

trait Path {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value>;
}

struct Handwritten {
    path: Vec<String>,
}

impl Path for Handwritten {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        let mut current = value;
        for key in self.path.iter() {
            current = match current.get(key) {
                Some(value) => value,
                None => return None,
            };
        }
        Some(current)
    }
}

struct SerdeJsonPath {
    expr: JsonPath,
}

impl Path for SerdeJsonPath {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        self.expr.query(value).first()
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let value = create_json();
    let hw = Handwritten {
        path: "sand.now.seeing.pound.method.motor.breathing.action.huge.relationship.fun.grade.express.music.which.everyone.fix.white.attached.break.bad.electricity.tail.title.prize.itself.completely.stared.lunch.particularly.plates.held.silver.seen.trail.cage.empty.from.sleep.lake.control.grain.changing.whispered.home.drove.add.widely.current.rubbed.whale.beneath.trail.explore.foot.salt.brought.drink.worth.dozen.solution.copper.moving.swing.neighbor.your.suit.were.harbor.forest.strike.favorite.start.concerned.draw.smallest.single.statement.favorite.stood.mill.freedom.basis.carry.naturally.parts.out.major.visit.appearance.sit.becoming.party.object.its.track.space.facing.experience".trim().split(".").map(|a| a.trim().to_owned()).collect::<Vec<_>>(),
    };
    let jp = SerdeJsonPath {
        expr: JsonPath::parse("$.sand.now.seeing.pound.method.motor.breathing.action.huge.relationship.fun.grade.express.music.which.everyone.fix.white.attached.break.bad.electricity.tail.title.prize.itself.completely.stared.lunch.particularly.plates.held.silver.seen.trail.cage.empty.from.sleep.lake.control.grain.changing.whispered.home.drove.add.widely.current.rubbed.whale.beneath.trail.explore.foot.salt.brought.drink.worth.dozen.solution.copper.moving.swing.neighbor.your.suit.were.harbor.forest.strike.favorite.start.concerned.draw.smallest.single.statement.favorite.stood.mill.freedom.basis.carry.naturally.parts.out.major.visit.appearance.sit.becoming.party.object.its.track.space.facing.experience").unwrap(),
    };

    assert_eq!(hw.get_path(&value), jp.get_path(&value));
    c.bench_function("Hand Optimized", |b| {
        b.iter(|| black_box(hw.get_path(&value)))
    });
    c.bench_function("JsonPathBased", |b| {
        b.iter(|| black_box(jp.get_path(&value)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
