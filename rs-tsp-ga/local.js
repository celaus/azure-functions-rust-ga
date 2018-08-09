const rust = require("./wasm_tspsolver.js");

function createCities(n, minX, maxX, minY, maxY) {
    let _cities = new Array();
    for (let i = 0; i < n; i++) {
        let x = minX + Math.random() * (maxX - minX);
        let y = minY + Math.random() * (maxY - minY);
        _cities.push(Array.of(x, y));
    }
    return _cities;
}
let cities = createCities(30, 10, 690, 10, 580);

let x = cities.map((c) => c[0]);
let y = cities.map((c) => c[1]);
console.log("and go");
console.log(rust.sovle_tsp(x, y, 400, 100));

//const almost_pi = rust.approximate_pi(10000000);

//console.log(almost_pi);