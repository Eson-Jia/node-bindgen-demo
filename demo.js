const addon = require('./dist');
const assert = require("assert");

console.log(addon.sum(2,3));
//optional argument
console.log(addon.sum_optional(2));

addon.hello(2, (msg) => {
    assert.strictEqual(msg, "argument is: 2");
    console.log(msg);  // print out argument is 2
});