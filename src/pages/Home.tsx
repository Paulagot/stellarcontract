import React from "react";

const Home: React.FC = () => (
  <div style={{ padding: '2rem', maxWidth: '800px', margin: '0 auto' }}>
    <h1>Welcome to your app!</h1>
    <p>
      This is a basic template to get your dapp started with Stellar
      contracts. You can customize it further by adding your own contracts,
      components, and styles.
    </p>
    <h2>Developing your contracts</h2>
    <p>
      Your contracts are located in the contracts/ directory, and you can
      modify them to suit your needs.
    </p>
    <p>
      As you update them, the <code style={{ backgroundColor: '#f5f5f5', padding: '2px 4px', borderRadius: '3px' }}>stellar scaffold watch</code>{" "}
      command will automatically recompile them and update the dapp with the
      latest changes.
    </p>
    <h2>Interacting with contracts from the frontend</h2>
    <p>
      Scaffold stellar automatically builds your contract packages, and you can
      import them in your frontend code like this:
    </p>
    <pre style={{ backgroundColor: '#f5f5f5', padding: '1rem', borderRadius: '5px', overflow: 'auto' }}>
      <code>{`import quiz from "./contracts/quiz.ts";`}</code>
    </pre>
    <p>And then you can call the contract methods like this:</p>
    <pre style={{ backgroundColor: '#f5f5f5', padding: '1rem', borderRadius: '5px', overflow: 'auto' }}>
      <code>{`const roomConfig = await quiz.get_room_config({"room_id": "your_room_id"});`}</code>
    </pre>
    <p>
      By doing this, you can use the contract methods in your components. If
      your contract emits events, check out the{" "}
      <code style={{ backgroundColor: '#f5f5f5', padding: '2px 4px', borderRadius: '3px' }}>useSubscription</code> hook in the hooks/ folder to
      listen to them.
    </p>
    <h2>Interacting with wallets</h2>
    <p>
      This project is already integrated with Stellar Wallet Kit, and the
      {` useWallet `} hook is available for you to use in your components. You
      can use it to connect to get connected account information.
    </p>
    <h2>Deploying your app</h2>
    <p>
      To deploy your contracts, use the{" "}
      <code style={{ backgroundColor: '#f5f5f5', padding: '2px 4px', borderRadius: '3px' }}>stellar contract deploy</code> command (
      <a href="https://developers.stellar.org/docs/build/guides/cli/install-deploy">
        docs
      </a>
      ) to deploy to the appropriate Stellar network.
    </p>
    <p>
      Build your frontend application code with{" "}
      <code style={{ backgroundColor: '#f5f5f5', padding: '2px 4px', borderRadius: '3px' }}>npm run build</code> and deploying the output in the
      <code style={{ backgroundColor: '#f5f5f5', padding: '2px 4px', borderRadius: '3px' }}>dist/</code> directory.
    </p>
  </div>
);

export default Home;