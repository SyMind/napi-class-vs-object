const {
    createObject1,
    createObject2,
    createObject3,
    createObject4,
    createClass1,
    createClass2,
    createClass3,
    createClass4,
    setObject1,
    setObject2,
    setObject3,
    setObject4,
    setClass1,
    setClass2,
    setClass3,
    setClass4,
} = require(".");

console.time("object with 1 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject1();
    o.f1;
}
console.timeEnd("object with 1 field");

console.time("object with 2 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject2();
    o.f1;
}
console.timeEnd("object with 2 field");

console.time("object with 3 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject3();
    o.f1;
}
console.timeEnd("object with 3 field");

console.time("object with 4 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject4();
    o.f1;
}
console.timeEnd("object with 4 field");



console.time("class with 1 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass1();
    o.f1;
}
console.timeEnd("class with 1 field");

console.time("class with 2 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass2();
    o.f1;
}
console.timeEnd("class with 2 field");

console.time("class with 3 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass3();
    o.f1;
}
console.timeEnd("class with 3 field");

console.time("class with 4 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass4();
    o.f1;
}
console.timeEnd("class with 4 field");



console.time("class with 4 fields read 2 fields");
for (let i = 0; i < 10000; i++) {
    let c = createClass4();
    c.f1;
    c.f2;
}
console.timeEnd("class with 4 fields read 2 fields");

console.time("class with 4 fields read 3 fields");
for (let i = 0; i < 10000; i++) {
    let c = createClass4();
    c.f1;
    c.f2;
    c.f3;
}
console.timeEnd("class with 4 fields read 3 fields");

console.time("class with 4 fields read 4 fields");
for (let i = 0; i < 10000; i++) {
    let c = createClass4();
    c.f1;
    c.f2;
    c.f3;
    c.f4;
}
console.timeEnd("class with 4 fields read 4 fields");


console.time("object with 1 fields and set");
for (let i = 0; i < 10000; i++) {
    let c = createObject1();
    setObject1(c);
}
console.timeEnd("object with 1 fields and set");

console.time("object with 2 fields and set");
for (let i = 0; i < 10000; i++) {
    let c = createObject2();
    setObject2(c);
}
console.timeEnd("object with 2 fields and set");

console.time("object with 3 fields and set");
for (let i = 0; i < 10000; i++) {
    let c = createObject3();
    setObject3(c);
}
console.timeEnd("object with 3 fields and set");

console.time("object with 4 fields and set");
for (let i = 0; i < 10000; i++) {
    let c = createObject4();
    setObject4(c);
}
console.timeEnd("object with 4 fields and set");



console.time("class with 1 fields and set");
for (let i = 0; i < 10000; i++) {
    let c = createClass1();
    setClass1(c);
}
console.timeEnd("class with 1 fields and set");

console.time("class with 2 fields and set");
for (let i = 0; i < 10000; i++) {
    let c = createClass2();
    setClass2(c);
}
console.timeEnd("class with 2 fields and set");

console.time("class with 3 fields and set");
for (let i = 0; i < 10000; i++) {
    let c = createClass3();
    setClass3(c);
}
console.timeEnd("class with 3 fields and set");

console.time("class with 4 fields and set");
for (let i = 0; i < 10000; i++) {
    let c = createClass4();
    setClass4(c);
}
console.timeEnd("class with 4 fields and set");
