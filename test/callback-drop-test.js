const { init, SteamCallback } = require('../index.js')

console.log('Testing callback drop prevention...')

const client = init(480);

// Test 1: Register a callback WITHOUT storing the handle (simulating the issue)
console.log('\n=== Test 1: Register callback without storing handle ===')
client.callback.register(SteamCallback.PersonaStateChange, (value) => {
    console.log('Test 1 - PersonaStateChange callback fired:', value)
})
console.log('Callback registered without storing handle. It should remain active.')

// Test 2: Register a callback WITH storing the handle
console.log('\n=== Test 2: Register callback with stored handle ===')
const handle2 = client.callback.register(SteamCallback.SteamServersConnected, (value) => {
    console.log('Test 2 - SteamServersConnected callback fired:', value)
})
console.log('Callback registered with stored handle.')

// Test 3: Explicitly disconnect a callback
console.log('\n=== Test 3: Register and disconnect callback ===')
const handle3 = client.callback.register(SteamCallback.SteamServersDisconnected, (value) => {
    console.log('Test 3 - SteamServersDisconnected callback fired (should not happen):', value)
})
setTimeout(() => {
    console.log('Disconnecting handle3...')
    handle3.disconnect()
    console.log('Handle3 disconnected. Callback should no longer fire.')
}, 1000)

// Force garbage collection if available (with --expose-gc flag)
setTimeout(() => {
    if (global.gc) {
        console.log('\nForcing garbage collection...')
        global.gc()
        console.log('GC complete. Callbacks without stored handles should still be active.')
    } else {
        console.log('\nNote: Run with --expose-gc flag to test garbage collection')
    }
}, 2000)

// Keep process alive for a bit to observe callbacks
setTimeout(() => {
    console.log('\n=== Test Complete ===')
    console.log('If Test 1 callback remained active after GC, the fix is working!')
    process.exit(0)
}, 5000)
