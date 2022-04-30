// index.js

// Index file helps in contract deployment
module.exports = ({ wallets, refs, config, client }) => ({
  getCount: () => client.query("counter", { get_count: {} }), 
  increment: (signer = wallets.validator) => 
    client.execute(signer, "counter", { increment: {} }), 
  
    // This is what we're adding at last to the contract
    getCristianoRonaldo: () => client.query("clicker", { get_speed: {} }), 
})