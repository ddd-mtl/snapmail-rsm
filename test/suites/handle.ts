const { config, ALEX_NICK, BILLY_NICK, CAMILLE_NICK } = require('../config')

// -- Export scenarios -- //

module.exports = scenario => {
    //scenario("test get/set handle", test_getset_handle)
    scenario("test handle list", test_handle_list)

    // FAILING
    // scenario("test set 3 handles", test_set_3_handles)
}

const delay = (ms) => new Promise((r) => setTimeout(r, ms));


// -- Scenarios -- //

/**
 *
 */
const test_getset_handle = async (s, t) => {
    // -- Setup conductor
    const { conductor } = await s.players({ conductor: config })
    await conductor.spawn()
    // -- Get Ids
    const [_dnaHash, alexAddress] = conductor.cellId(ALEX_NICK)
    console.log({alexAddress})
    const [_dnaHash2, billyAddress] = conductor.cellId(BILLY_NICK)
    console.log({billyAddress})

    //console.log(alex)
    const name = "alex"
    const handle_address = await conductor.call(ALEX_NICK, "snapmail", "set_handle", name)
    console.log('handle_address: ' + JSON.stringify(handle_address))
    //console.log({handle_address})
    console.log('handle_address.hash: ' + handle_address.hash)
    //t.deepEqual(result.Ok, name)
    //t.match(handle_address.hash, RegExp('Qm*'))

    await delay(10);

    //let playerArray = new Array(alex, billy)
    //const succeeded = await s.simpleConsistency("__snapmail", playerArray)

    const result = await conductor.call(ALEX_NICK, "snapmail", "get_my_handle", undefined)
    console.log('result1: ' + JSON.stringify(result))
    t.deepEqual(result, name)

    //const params2 = { agentId: alexAddress }
    const result2 = await conductor.call(ALEX_NICK, "snapmail", "get_handle", alexAddress)
    console.log('result2: ' + JSON.stringify(result2))
    t.deepEqual(result2, name)

    const result3 = await conductor.call(BILLY_NICK, "snapmail", "get_handle", alexAddress)
    console.log('result3: ' + JSON.stringify(result3))
    t.deepEqual(result3, name)

    // -- Ping -- //

    const result4 = await conductor.call(BILLY_NICK, "snapmail", "ping_agent", alexAddress)
    console.log('result4: ' + JSON.stringify(result4))
    t.deepEqual(result4, true)
};


/**
 *
 *
const test_handle_list = async (s, t) => {
    // -- Setup conductor
    const { conductor } = await s.players({ conductor: config })
    await conductor.spawn()
    // -- Get Ids
    const [_dnaHash, alexAddress] = conductor.cellId(ALEX_NICK)
    console.log({alexAddress})
    const [_dnaHash2, billyAddress] = conductor.cellId(BILLY_NICK)
    console.log({billyAddress})
    const [_dnaHash3, camilleAddress] = conductor.cellId(CAMILLE_NICK)
    console.log({camilleAddress})

    // Set Alex
    let name = "alex"
    let params = { name }
    let handle_address = await conductor.call(ALEX_NICK, "snapmail", "set_handle", params)
    console.log('handle_address1: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    await delay(10);

    // Set billy
    name = "billy"
    params = { name }
    handle_address = await conductor.call(BILLY_NICK, "snapmail", "set_handle", params)
    console.log('handle_address2: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    await delay(10);


    let result = await conductor.call(BILLY_NICK, "snapmail", "get_all_handles", {})
    console.log('handle_list: ' + JSON.stringify(result))
    t.deepEqual(result.length, 2)

    // Set camille
    name = "camille"
    handle_address = await conductor.call(CAMILLE_NICK, "snapmail", "set_handle", name)
    console.log('handle_address3: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    await delay(10);

    result = await conductor.call(BILLY_NICK, "snapmail", "get_all_handles", {})
    console.log('handle_list: ' + JSON.stringify(result))
    t.deepEqual(result.length, 3)

    // Update Billy
    name = "bob"
    handle_address = await conductor.call(BILLY_NICK, "snapmail", "set_handle", name)
    console.log('handle_address4: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    await delay(10);

    result = await conductor.call(BILLY_NICK, "snapmail", "get_all_handles", {})
    console.log('handle_list updated: ' + JSON.stringify(result))
    t.deepEqual(result.length, 3)
};
*/

/**
 *  TODO: Currently this fails as Holochain doesnt allow multiple updates of an entry in one call
 *
const test_set_3_handles = async (s, t) => {
    const {alex} = await s.players({alex: conductorConfig}, true)

    const name = "joe"
    const params0 = { name }
    const handle_address0 = await alex.call("app", "snapmail", "set_handle", params0)
    console.log('handle_address0: ' + JSON.stringify(handle_address0))
    t.match(handle_address0.Ok, RegExp('Qm*'))

    const name1 = "alex"
    const name2 = "billy"
    const name3 = "bob"
    const params = { name1, name2, name3 }
    const handle_address = await alex.call("app", "snapmail", "set_three_handles", params)
    console.log('handle_address: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))

    let result = await alex.call("app", "snapmail", "get_my_handle", {})
    console.log('result1: ' + JSON.stringify(result))
    t.deepEqual(result.Ok, name3)

    // Get history
    let address = handle_address.Ok
    let params42 = { address }
    let history_result = await alex.call("app", "snapmail", "get_my_handle_history", params42)
    console.log('history_result: ' + JSON.stringify(history_result))
    t.deepEqual(history_result.length, 3)
};
*/