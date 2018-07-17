const {
  Citizen,
  Population
} = require('genometrics');
const distance = require('euclidean-distance')

function totalDistance(points) {
  let total = 0;
  for (let i = 1; i < points.length; i++) {
    total += distance(points[i - 1], points[i]);
  }
  return total;
}

Citizen.prototype.calculateFitness = function () {
  this.fitness = -totalDistance(createTour(this.genome));
};

function shuffle(array) {
  var currentIndex = array.length,
    temporaryValue, randomIndex;

  // While there remain elements to shuffle...
  while (0 !== currentIndex) {

    // Pick a remaining element...
    randomIndex = Math.floor(Math.random() * currentIndex);
    currentIndex -= 1;

    // And swap it with the current element.
    temporaryValue = array[currentIndex];
    array[currentIndex] = array[randomIndex];
    array[randomIndex] = temporaryValue;
  }

  return array;
}

function createPopulation(cities, popsize) {
  let population = new Population();
  let indices = cities.map((c, i) => `${i}`);
  for (let m = 0; m < popsize; m++) {
    let genome = shuffle(indices.slice());
    let citizen = new Citizen(genome);
    population.members.push(citizen);
  }
  return population;
}

function solve(cities, generations, mutationRate, populationSize) {
  let pop = createPopulation(cities, populationSize);

  pop.members.forEach(citizen => {
    citizen.calculateFitness();
  });

  for (let _ = 0; _ < generations; _++) {
    pop.selection();
    pop.crossover();
    pop.mutation(mutationRate, citizen => {
      let genLength = citizen.genome.length;
      for (let i = 0; i < genLength; i++) {
        if (Math.random() < 0.05) {
          let swapIdx = Math.floor(Math.random() * (genLength - 2));
          if (swapIdx >= i) {
            swapIdx += 1;
          }
          let tmp = citizen.genome[i];
          citizen.genome[i] = citizen.genome[swapIdx];
          citizen.genome[swapIdx] = tmp;
        }
      }
    });

    let _sumFitness = 0;
    let _minFitness = 1000000000000;
    let _maxFitness = -_minFitness;
    let best =
      pop.members.forEach(citizen => {
        citizen.calculateFitness();
        _sumFitness += citizen.fitness;
        _minFitness = Math.min(_minFitness, citizen.fitness);
        _maxFitness = Math.max(_maxFitness, citizen.fitness);

      });
    let averageFitness = _sumFitness / pop.members.length;
    //console.log(`${_},${_sumFitness / pop.members.length},${_minFitness},${_maxFitness}`);
    return pop.members.reduce((p, c) => {
      return c.fitness > p.fitness ? c : p
    }, pop.members[0]);
  }
}

module.exports = function (context, req) {
  if (req.body.tour) {
    const tour = req.body.tour
    let best = solve(tour, 0.2, 400, 100);

    context.res = {
      status: 200,
      body: `${JSON.stringify(best.map((t) => ({x: t[0], y: t[1]})))}`
    };
  } else {
    context.res = {
      status: 400,
    };
  }
  context.done();
};