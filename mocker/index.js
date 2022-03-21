const { faker } = require('@faker-js/faker');


Array.prototype.rand = function(){
    return this[Math.floor(Math.random()*this.length)];
  }

const randSteam = new Array(50).fill(`steam:${faker.random.uuid()}`)
const randLicenses = new Array(50).fill(`license:${faker.random.uuid()}`)
const actionTypes = ["ban", "warn", "whitelist"]

const createPlayer = () => ({
    license: randSteam.rand().replace("steam:", ""),
    name: faker.name.lastName(),
    tsLastConnection: faker.date.past().getTime(),
    tsJoined: faker.date.past().getTime(),
    playTime: faker.random.number({min: 1}),
    notes: {}
})

const createAction = () => ({
    id: faker.random.uuid(),
    identifiers: [randSteam.rand(), randLicenses.rand()],
    type: actionTypes.rand(),
    author: faker.name.lastName(),
    reason: faker.git.commitMessage(),
    timestamp: faker.date.past().getMilliseconds(),
    revocation: {},
    expiration: [true, false].rand(),
    playerName: faker.name.lastName()
})

const obj = {players: [], actions: []}

for (let i = 0; i <= 10000; i++) {
    obj.players.push(createPlayer())
}

for (let i = 0; i <= 50000; i++) {
    obj.actions.push(createAction())
}

const str = JSON.stringify(obj)
require("fs").writeFileSync("mock.json", str)

