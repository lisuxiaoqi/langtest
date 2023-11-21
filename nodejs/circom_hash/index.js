const { Scalar } = require("ffjavascript");
const ethers = require("ethers")
const circomlib=require("circomlib")

function buildSpotWithdraw(tx) {
    let res = Scalar.e(0);
    console.log("zero:", res)
    res = Scalar.add(res, tx.user_id || 0);
    console.log("uid", res)
    bs = Scalar.shl(tx.business_type || 0, 48)
    console.log("business_type", bs)
    res = Scalar.add(res, bs);
    ts = Scalar.shl(tx.token_id || 0, 64)
    console.log("token_id", ts)
    res = Scalar.add(res, ts);

    fm = formatAmount(tx.amount)
    console.log("formatAmount origin:", fm)
    fms = Scalar.shl(fm, 80)
    console.log("formatAmount shift:", fms)
    res = Scalar.add(
        res,
        fms
    );
    console.log("final", res)
    const h = circomlib.poseidon([res]);
    console.log("poseidon", h)
    return h;
}

function formatAmount(str, dis) {
    console.log("origin amount", str)
    console.log("toString amount", str.toString())
    return ethers.parseUnits(str.toString(), 20)
}

class Tx{
    constructor(uid, bt, tid, amount) {
        this.user_id = uid;
        this.business_type = bt;
        this.token_id = tid;
        this.amount = amount
    }
}

const tx = new Tx(50000007, 1, 257, 8)
buildSpotWithdraw(tx)