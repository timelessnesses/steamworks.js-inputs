const { init } = require('../index.js')

const client = init(3782120)
client.input.init()
setInterval(() => {
    // IMPORTANT: call runFrame once per tick BEFORE reading controller states
    client.input.runFrame()

    const controllers = client.input.getControllers()[0]
    console.log('Controllers: ' + controllers)
    if (!controllers) return
    console.log('Triggering a Lightbar pulse...')
    controllers.setLightbarColor(Math.floor(Math.random() * 255), Math.floor(Math.random() * 255), Math.floor(Math.random() * 255), false)
    console.log('Done')
}, 200)