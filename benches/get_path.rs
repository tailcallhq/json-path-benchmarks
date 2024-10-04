use criterion::{black_box, Criterion};
use jsonpath::Selector;
use jsonpath_rs::json_path::{DummyTrackerGenerator, PathCalculator, Query};
use serde_json::Value;
use std::str::FromStr;

fn create_json() -> Value {
    let json = include_str!("./data.json");

    serde_json::from_str(json).unwrap()
}

trait Path {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value>;
}

struct JsonPathRs<'a> {
    path: PathCalculator<'a, DummyTrackerGenerator>,
}

impl<'a> JsonPathRs<'a> {
    fn new(query: &'a Query) -> Self {
        let path = jsonpath_rs::create(query);
        Self { path }
    }
}

impl Path for JsonPathRs<'_> {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        self.path.calc(value).first().map(|v| *v)
    }
}

struct Handwritten {
    path: Vec<String>,
}

impl Default for Handwritten {
    fn default() -> Self {
        Handwritten {
            path: "sand.now.seeing.pound.method.motor.breathing.action.huge.relationship.fun.grade.express.music.which.everyone.fix.white.attached.break.bad.electricity.tail.title.prize.itself.completely.stared.lunch.particularly.plates.held.silver.seen.trail.cage.empty.from.sleep.lake.control.grain.changing.whispered.home.drove.add.widely.current.rubbed.whale.beneath.trail.explore.foot.salt.brought.drink.worth.dozen.solution.copper.moving.swing.neighbor.your.suit.were.harbor.forest.strike.favorite.start.concerned.draw.smallest.single.statement.favorite.stood.mill.freedom.basis.carry.naturally.parts.out.major.visit.appearance.sit.becoming.party.object.its.track.space.facing.experience".trim().split(".").map(|a| a.trim().to_owned()).collect::<Vec<_>>(),
        }
    }
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

struct JsonPath {
    expr: Selector,
}

impl Default for JsonPath {
    fn default() -> Self {
        let selector = Selector::new("$.sand.now.seeing.pound.method.motor.breathing.action.huge.relationship.fun.grade.express.music.which.everyone.fix.white.attached.break.bad.electricity.tail.title.prize.itself.completely.stared.lunch.particularly.plates.held.silver.seen.trail.cage.empty.from.sleep.lake.control.grain.changing.whispered.home.drove.add.widely.current.rubbed.whale.beneath.trail.explore.foot.salt.brought.drink.worth.dozen.solution.copper.moving.swing.neighbor.your.suit.were.harbor.forest.strike.favorite.start.concerned.draw.smallest.single.statement.favorite.stood.mill.freedom.basis.carry.naturally.parts.out.major.visit.appearance.sit.becoming.party.object.its.track.space.facing.experience").unwrap();
        Self { expr: selector }
    }
}

impl Path for JsonPath {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        let titles = self.expr.find(&value).collect::<Vec<_>>();
        titles.first().map(|v| *v)
    }
}

struct SerdeJsonPath {
    expr: serde_json_path::JsonPath,
}

impl Default for SerdeJsonPath {
    fn default() -> Self {
        SerdeJsonPath {
            expr: serde_json_path::JsonPath::parse("$.sand.now.seeing.pound.method.motor.breathing.action.huge.relationship.fun.grade.express.music.which.everyone.fix.white.attached.break.bad.electricity.tail.title.prize.itself.completely.stared.lunch.particularly.plates.held.silver.seen.trail.cage.empty.from.sleep.lake.control.grain.changing.whispered.home.drove.add.widely.current.rubbed.whale.beneath.trail.explore.foot.salt.brought.drink.worth.dozen.solution.copper.moving.swing.neighbor.your.suit.were.harbor.forest.strike.favorite.start.concerned.draw.smallest.single.statement.favorite.stood.mill.freedom.basis.carry.naturally.parts.out.major.visit.appearance.sit.becoming.party.object.its.track.space.facing.experience").unwrap(),
        }
    }
}

impl Path for SerdeJsonPath {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        self.expr.query(value).first()
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let value = create_json();

    let hw = Handwritten::default();
    let jp = JsonPath::default();
    let jp_serde = SerdeJsonPath::default();
    let jp_rs_selector = jsonpath_rs::compile("$.sand.now.seeing.pound.method.motor.breathing.action.huge.relationship.fun.grade.express.music.which.everyone.fix.white.attached.break.bad.electricity.tail.title.prize.itself.completely.stared.lunch.particularly.plates.held.silver.seen.trail.cage.empty.from.sleep.lake.control.grain.changing.whispered.home.drove.add.widely.current.rubbed.whale.beneath.trail.explore.foot.salt.brought.drink.worth.dozen.solution.copper.moving.swing.neighbor.your.suit.were.harbor.forest.strike.favorite.start.concerned.draw.smallest.single.statement.favorite.stood.mill.freedom.basis.carry.naturally.parts.out.major.visit.appearance.sit.becoming.party.object.its.track.space.facing.experience").unwrap();
    let jp_rs = JsonPathRs::new(&jp_rs_selector);

    assert_eq!(hw.get_path(&value), jp.get_path(&value));
    assert_eq!(hw.get_path(&value), jp_serde.get_path(&value));
    assert_eq!(hw.get_path(&value), jp_rs.get_path(&value));
    c.bench_function("Hand Optimized", |b| {
        b.iter(|| black_box(hw.get_path(&value)))
    });
    c.bench_function("JsonPath", |b| {
        b.iter(|| black_box(jp_serde.get_path(&value)))
    });

    c.bench_function("SerdeJsonPath", |b| {
        b.iter(|| black_box(jp_serde.get_path(&value)))
    });

    // c.bench_function("JsonPathRust", |b| {
    //     b.iter(|| black_box(jp_rust.get_path(&value)))
    // });

    c.bench_function("JsonPathRs", |b| {
        b.iter(|| black_box(jp_rs.get_path(&value)))
    });
}
