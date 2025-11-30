const { init, shutdownClient } = require('../index.js')
const client = init(480);

const timmy = client.friends.requestUserInformation(BigInt("76561199213195978"), true, 5).then(user => {
    console.log(user);
    return user;
});
const whoeverthisis = client.friends.requestUserInformation(BigInt("76561198344665803"), true, 5).then(user => {
    console.log(user);
    return user;
});

console.log(timmy);
console.log(whoeverthisis);
client.shutdownClient();