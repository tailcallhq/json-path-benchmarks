use criterion::{black_box, Criterion};
use jsonpath::Selector;
use jsonpath_rs::json_path::{self, Query};
use serde_json::Value;

const QUERY: &str = "sand.now.seeing.pound.method.motor.breathing.action.huge.relationship.fun.grade.express.music.which.everyone.fix.white.attached.break.bad.electricity.tail.title.prize.itself.completely.stared.lunch.particularly.plates.held.silver.seen.trail.cage.empty.from.sleep.lake.control.grain.changing.whispered.home.drove.add.widely.current.rubbed.whale.beneath.trail.explore.foot.salt.brought.drink.worth.dozen.solution.copper.moving.swing.neighbor.your.suit.were.harbor.forest.strike.favorite.start.concerned.draw.smallest.single.statement.favorite.stood.mill.freedom.basis.carry.naturally.parts.out.major.visit.appearance.sit.becoming.party.object.its.track.space.facing.experience";

fn create_json() -> Value {
    let json = include_str!("./data.json");
    serde_json::from_str(json).unwrap()
}

fn query() -> String {
    format!("$.{}", QUERY)
}

trait Path {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value>;
}

struct JsonPathRs<'a> {
    query: Query<'a>,
}

impl<'a> JsonPathRs<'a> {
    fn new(query: Query<'a>) -> Self {
        Self { query }
    }
}

impl Path for JsonPathRs<'_> {
    fn get_path<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        jsonpath_rs::create(&self.query)
            .calc(value)
            .first()
            .map(|v| *v)
    }
}

struct Handwritten {
    path: Vec<String>,
}

impl Handwritten {
    fn new(path: Vec<String>) -> Self {
        Handwritten { path }
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

impl JsonPath {
    fn new(query: String) -> Self {
        let selector = Selector::new(query.as_str()).unwrap();
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

impl SerdeJsonPath {
    fn new(query: String) -> Self {
        SerdeJsonPath {
            expr: serde_json_path::JsonPath::parse(query.as_str()).unwrap(),
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

    let hand_written = Handwritten::new(
        QUERY
            .trim()
            .split(".")
            .map(|a| a.to_owned())
            .collect::<Vec<_>>(),
    );
    let i1 = JsonPath::new(query());
    let i2 = SerdeJsonPath::new(query());
    let q = query();
    let i3 = JsonPathRs::new(jsonpath_rs::compile(q.as_str()).unwrap());

    assert_eq!(hand_written.get_path(&value), i1.get_path(&value));
    assert_eq!(hand_written.get_path(&value), i2.get_path(&value));
    assert_eq!(hand_written.get_path(&value), i3.get_path(&value));
    c.bench_function("HandWritten", |b| {
        b.iter(|| black_box(hand_written.get_path(&value)))
    });
    c.bench_function("JsonPath", |b| b.iter(|| black_box(i2.get_path(&value))));
    c.bench_function("SerdeJsonPath", |b| {
        b.iter(|| black_box(i2.get_path(&value)))
    });
    c.bench_function("JsonPathRs", |b| b.iter(|| black_box(i3.get_path(&value))));
}
