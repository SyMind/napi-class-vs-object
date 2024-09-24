const fs = require('fs');
const {
    createObject1,
    createObject2,
    createObject3,
    createObject4,
    createObject5,
    createObject6,
    createObject7,
    createObject8,
    createObject9,
    createObject10,
    createClass1,
    createClass2,
    createClass3,
    createClass4,
    createClass5,
    createClass6,
    createClass7,
    createClass8,
    createClass9,
    createClass10,
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
}
console.timeEnd("object with 1 field");

console.time("object with 2 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject2();
}
console.timeEnd("object with 2 field");

console.time("object with 3 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject3();
}
console.timeEnd("object with 3 field");

console.time("object with 4 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject4();
    o.f1;
}
console.timeEnd("object with 4 field");

console.time("object with 5 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject5();
}
console.timeEnd("object with 5 field");

console.time("object with 6 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject6();
}
console.timeEnd("object with 6 field");

console.time("object with 7 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject7();
}
console.timeEnd("object with 7 field");

console.time("object with 8 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject8();
}
console.timeEnd("object with 8 field");

console.time("object with 9 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject9();
}
console.timeEnd("object with 9 field");

console.time("object with 10 field");
for (let i = 0; i < 10000; i++) {
    let o = createObject10();
}
console.timeEnd("object with 10 field");







console.time("class with 1 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass1();
}
console.timeEnd("class with 1 field");

console.time("class with 2 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass2();
}
console.timeEnd("class with 2 field");

console.time("class with 3 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass3();
}
console.timeEnd("class with 3 field");

console.time("class with 4 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass4();
}
console.timeEnd("class with 4 field");

console.time("class with 5 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass5();
}
console.timeEnd("class with 5 field");

console.time("class with 6 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass6();
}
console.timeEnd("class with 6 field");

console.time("class with 7 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass7();
}
console.timeEnd("class with 7 field");

console.time("class with 8 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass8();
}
console.timeEnd("class with 8 field");

console.time("class with 9 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass9();
}
console.timeEnd("class with 9 field");

console.time("class with 10 field");
for (let i = 0; i < 10000; i++) {
    let o = createClass10();
}
console.timeEnd("class with 10 field");




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
