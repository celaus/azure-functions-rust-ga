const {
  Citizen,
  Population
} = require('genometrics');

const A_HIGH_NR = 1000000000000;

function x(s, t) {
  let _map1 = {};
  let _map2 = {};

  const x1 = Math.floor(Math.random() * (s.length - 1));
  const x2 = x1 + Math.floor(Math.random() * (s.length - x1));

  let offspring = [Array.from(s), Array.from(t)];

  for (let i = x1; i < x2; i++) {
    offspring[0][i] = t[i];
    _map1[t[i]] = s[i];

    offspring[1][i] = s[i];
    _map2[s[i]] = t[i];
  }

  for (let i = 0; i < x1; i++) {
    while (offspring[0][i] in _map1) {
      offspring[0][i] = _map1[offspring[0][i]];
    }
    while (offspring[1][i] in _map2) {
      offspring[1][i] = _map2[offspring[1][i]];
    }
  }

  for (let i = x2; i < s.length; i++) {
    while (offspring[0][i] in _map1) {
      offspring[0][i] = _map1[offspring[0][i]];
    }
    while (offspring[1][i] in _map2) {
      offspring[1][i] = _map2[offspring[1][i]];
    }
  }
  return offspring;
}

function PMXCrossover(pop) {
  const p = pop.members;
  const rounds = p.length % 2 ? p.length - 1 : p.length;
  for (let i = 1; i < rounds; i += 2) {
    const s = p[i].genome;
    const t = p[i - 1].genome;
    x(s, t).forEach((offspring) => {
      if (new Set(offspring).size != _cities.length) {
        console.log(offspring);
        throw new Error("nope");
      }
      pop.members.push(new Citizen(offspring));
    });

  }
}


function distance(p1, p2) {
  return Math.sqrt(Math.pow(p1[0] - p2[0], 2) + Math.pow(p1[1] - p2[1], 2));
}

let _cities = [];

function createTour(idx) {
  return idx.map((e) => _cities[e]);
}

function totalDistance(points) {
  let total = 0;
  for (let i = 1; i < points.length; i++) {
    total += distance(points[i - 1], points[i]);;
  }
  return total;
}

Citizen.prototype.calculateFitness = function () {
  if (new Set(this.genome).size < this.genome.length) {
    this.fitness = -A_HIGH_NR;
  } else {
    this.fitness = -totalDistance(createTour(this.genome));
  }
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
  _cities = cities;
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

  let history = [];

  for (let _ = 0; _ < generations; _++) {
    pop.selection();
    PMXCrossover(pop);
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
    let _minFitness = A_HIGH_NR;
    let _maxFitness = -_minFitness;
    pop.members.forEach(citizen => {
      citizen.calculateFitness();
      _sumFitness += citizen.fitness;
      _minFitness = Math.min(_minFitness, citizen.fitness);
      _maxFitness = Math.max(_maxFitness, citizen.fitness);
    });
    let averageFitness = Math.abs(_sumFitness / pop.members.length);
    history.push(averageFitness);
    //  console.log(`${_},${_sumFitness / pop.members.length},${_minFitness},${_maxFitness}`);
  }

  return {
    citizen: pop.members.reduce((p, c) => {
      return c.fitness > p.fitness ? c : p
    }, pop.members[0]),
    history: history
  };
}

module.exports = function (context, req) {
  if (req.body && req.body.tour) {
    const tour = req.body.tour.map((t) => ([t.x, t.y]));
    console.log("Solving ... ");
    const start = new Date();
    let solution = solve(tour, 400, 0.2, 100);
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