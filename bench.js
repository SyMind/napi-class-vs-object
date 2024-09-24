const percentile = require('percentile');
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

console.log("object with 1 field", measure(() => {
    const o = createObject1();
}));
console.log("object with 2 field", measure(() => {
    const o = createObject2();
}));
console.log("object with 3 field", measure(() => {
    const o = createObject3();
}));
console.log("object with 4 field", measure(() => {
    const o = createObject4();
}));
console.log("object with 5 field", measure(() => {
    const o = createObject5();
}));
console.log("object with 6 field", measure(() => {
    const o = createObject6();
}));
console.log("object with 7 field", measure(() => {
    const o = createObject7();
}));
console.log("object with 8 field", measure(() => {
    const o = createObject8();
}));
console.log("object with 9 field", measure(() => {
    const o = createObject9();
}));
console.log("object with 10 field", measure(() => {
    const o = createObject10();
}));





console.log("class with 1 field", measure(() => {
    const o = createClass1();
}));
console.log("class with 2 field", measure(() => {
    const o = createClass2();
}));
console.log("class with 3 field", measure(() => {
    const o = createClass3();
}));
console.log("class with 4 field", measure(() => {
    const o = createClass4();
}));
console.log("class with 5 field", measure(() => {
    const o = createClass5();
}));
console.log("class with 6 field", measure(() => {
    const o = createClass6();
}));
console.log("class with 7 field", measure(() => {
    const o = createClass7();
}));
console.log("class with 8 field", measure(() => {
    const o = createClass8();
}));
console.log("class with 9 field", measure(() => {
    const o = createClass9();
}));
console.log("class with 10 field", measure(() => {
    const o = createClass10();
}));




console.log("read all class with 1 field", measure(() => {
    const o = createClass1();
    o.f1;
}));
console.log("read all class with 2 field", measure(() => {
    const o = createClass2();
    o.f1;
    o.f2;
}));
console.log("read all class with 3 field", measure(() => {
    const o = createClass3();
    o.f1;
    o.f2;
    o.f3;
}));
console.log("read all class with 4 field", measure(() => {
    const o = createClass4();
    o.f1;
    o.f2;
    o.f3;
    o.f4;
}));
console.log("read all class with 5 field", measure(() => {
    const o = createClass5();
    o.f1;
    o.f2;
    o.f3;
    o.f4;
    o.f5;
}));
console.log("read all class with 6 field", measure(() => {
    const o = createClass6();
    o.f1;
    o.f2;
    o.f3;
    o.f4;
    o.f5;
    o.f6;
}));
console.log("read all class with 7 field", measure(() => {
    const o = createClass7();
    o.f1;
    o.f2;
    o.f3;
    o.f4;
    o.f5;
    o.f6;
    o.f7;
}));
console.log("read all class with 8 field", measure(() => {
    const o = createClass8();
    o.f1;
    o.f2;
    o.f3;
    o.f4;
    o.f5;
    o.f6;
    o.f7;
    o.f8;
}));
console.log("read all class with 9 field", measure(() => {
    const o = createClass9();
    o.f1;
    o.f2;
    o.f3;
    o.f4;
    o.f5;
    o.f6;
    o.f7;
    o.f8;
    o.f9;
}));
console.log("read all class with 10 field", measure(() => {
    const o = createClass10();
    o.f1;
    o.f2;
    o.f3;
    o.f4;
    o.f5;
    o.f6;
    o.f7;
    o.f8;
    o.f9;
    o.f10;
}));

function measure(fn) {
    const times = [];
    for (let i = 0; i < 10000; i++) {
        const start = performance.now();
        fn();
        const end = performance.now();
        times.push(end - start);
    }
    // 单位 ns
    const ms = percentile(90, times);
    return Math.round(ms * 1000 * 1000);
}
