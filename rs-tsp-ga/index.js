const rust = require("./wasm_tspsolver.js"); 

module.exports = function (context, req) {
    if (req.body && req.body.tour) {
      const tour = req.body.tour;
      const x = tour.map((t) => (t.x));
      const y = tour.map((t) => (t.y));
      console.log("Solving ... ");
      const start = new Date();
      let _solution = rust.sovle_tsp(x, y, 400, 200);
      const elapsed = new Date().getTime() - start.getTime(); 
      console.log(`Solved TSP in ${elapsed} ms`);
      let solution = JSON.parse(_solution);
      let result = {
        tour: solution.citizen.map((t) => ({
          x: x[t],
          y: y[t]
        })),
        history: solution.history.map((h) => h * -1)
      };
      context.res = {
        status: 200,
        body: `${JSON.stringify(result)}`
      };
    } else {
      context.res = {
        status: 400,
      };
    }
    context.done();
  };