const { writeFileSync } = require('fs');
const { init, shutdown } = require('../index.js')
const client = init(480);

const timmy = client.friends.requestUserInformation(BigInt("76561199213195978"), false, 5).then(user => {
    console.log(user);
    writeFileSync("timmy.png", user.largeAvatar);
    return user;
});
const whoeverthisis = client.friends.requestUserInformation(BigInt("76561198028600619"), false, 1).then(user => {
    console.log(user);
    writeFileSync("whoeverthisis.png", user.largeAvatar);
    return user;
});

console.log(timmy);
console.log(whoeverthisis);

setTimeout(() => {
    console.log("Shutting down");
    shutdown();
}, 15000);
