function complexOperation(n) {
    let a = (n * 3) + Math.floor(Math.random() * 10);
    let b = (a ^ 0x5a5a5a5a) & 0xffff;
    let c = ((b << 3) | (b >> 5)) & 0xff;
    let d = (c + n * 7) % 1000;
    let e = d ^ (Math.floor(Math.random() * 256));
    return e;
}

let num = 1;

let key = null;

function calculateAndLogResult() {
    let result = complexOperation(num);
    console.log("result is... " + result)
    num += Math.floor(Math.random() * 3) + 1;

    key = result
}

calculateAndLogResult()

setInterval(calculateAndLogResult, 10000);