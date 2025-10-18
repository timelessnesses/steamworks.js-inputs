const path = require('path')
const { init } = require('../index.js')

const client = init(2234234)

// 1) Set manifest path BEFORE init (absolute path recommended)
const manifestPath = path.join(__dirname, '../aufh.vdf')
console.log('Using manifest:', manifestPath)
console.log(client.input.setInputActionManifestFilePath(manifestPath))

// 2) Init Steam Input after path is set
client.input.init()

// Optional: confirm handles are non-zero and print them
const actionset = client.input.getActionSet('gameplay')
console.log('ActionSet handle:', actionset) // should not be 0


const affirm = client.input.getDigitalAction('select')
const cancel = client.input.getDigitalAction('back')
const control = client.input.getAnalogAction('laser_stick')
const gyro = client.input.getAnalogAction('laser_gyro')

console.log('Affirm handle:', affirm, 'Cancel handle:', cancel, 'Control handle:', control, 'Gyro handle:', gyro)

setInterval(() => {
    // console.clear()

    // IMPORTANT: call runFrame once per tick BEFORE reading controller states
    client.input.runFrame()

    const controllers = client.input.getControllers()
    console.log('Controllers: ' + controllers.length)

    controllers.forEach(controller => {
        if (controller.getHandle() === 0n) return

        // activate sets here if needed (already activated globally)
        controller.activateActionSet(actionset)

        console.log('============')
        console.log('Handle: ' + controller.getHandle())
        console.log('Type: ' + controller.getType())

        const digitalOrigins = controller.getDigitalActionOrigins(actionset, affirm)
        const analogOrigins  = controller.getAnalogActionOrigins(actionset, control)

        console.log('Buttons origins:', digitalOrigins.length ? digitalOrigins : '(empty)')
        console.log('Analog origins:', analogOrigins.length ? analogOrigins : '(empty)')

        console.log("Affirm pressed:", controller.isDigitalActionPressed(affirm))
        console.log("Cancel pressed:", controller.isDigitalActionPressed(cancel))
        console.log("Analog vector:", JSON.stringify(controller.getAnalogActionVector(control)))
        console.log("Gyro motion:", JSON.stringify(controller.getMotionData()))
        console.log("Gyro as analog:", JSON.stringify(controller.getAnalogActionVector(gyro)))
        console.log("Triggering a Vibration...")
        controller.triggerVibration(1000, 1000)
        console.log("Done")
        controller.triggerVibration(0, 0)
    })
}, 1000)
