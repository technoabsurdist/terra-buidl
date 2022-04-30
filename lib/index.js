// index.js

// Index file helps in contract deployment
module.exports = ({ wallets, refs, config, client }) => ({
  getCount: () => client.query("counter", { get_count: {} }), 
  getScores: () => client.query("clicker", { get_scores: {} }), 

  upsertScore: (score, signer = wallets.validator) =>
    client.execute(signer, "clicker", { upsert_score: { score } }), 
}); 