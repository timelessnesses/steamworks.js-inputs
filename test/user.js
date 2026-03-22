const { init } = require('../index.js')

const client = init(480, (api) => {
    console.log('Name: ' + api.localplayer.getName())
    console.log('Level: ' + api.localplayer.getLevel())
    console.log('Ip Country: ' + api.localplayer.getIpCountry())
    console.log('Steam Id: ' + api.localplayer.getSteamId())
    return 1000 / 60
})

// client.shutdownClient();