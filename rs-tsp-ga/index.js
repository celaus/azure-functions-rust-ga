const rust = require("./wasm_tspsolver.js"); 


module.exports = function (context, req) {
    if (req.body && req.body.tour) {
      const x = req.body.tour.map((t) => (t.x));
      const y = req.body.tour.map((t) => (t.y));
      console.log("Solving ... ");
      const start = new Date();
      let solution = rust.sovle_tsp(x,y);
      const elapsed = new Date().getTime() - start.getTime(); 
      console.log(`Solved TSP in ${elapsed} ms`);
      let result = {
        tour: createTour(solution.citizen.genome).map((t) => ({
          x: t[0],
          y: t[1]
        })),
        history: solution.history
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