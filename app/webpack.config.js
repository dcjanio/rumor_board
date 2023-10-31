const path = require('path');

module.exports = {
    mode: 'development',
  entry: './src/index.js', // The main entry point of your app, adjust if needed
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist') // 'dist' is a common output directory, adjust if needed
  },
  module: {
    rules: [
      // You might have loaders here, like for processing JS with Babel or CSS with style-loader
    ]
  },
  plugins: [
    // Any required plugins go here
  ],
  // ... other configuration options
};
