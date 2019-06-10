const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const HotModuleReplacementPlugin = require('webpack').HotModuleReplacementPlugin;
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = {
	entry: './src/index.js',
	output: {
		filename: 'index.js',
		path: path.resolve(__dirname, 'bundle')
	},
	module: {
		rules: [
			{
				test: /\.js$/,
				exclude: /node_modules/,
				use: {
					loader: 'babel-loader'
				}
			},
			{
				test: /\.css$/,
				use: [
					'style-loader',
					'css-loader'
				]
			},
			{
				test: /\.(png|jpg|gif|svg)$/,
				use: [
					{
						loader: 'file-loader',
						options: {
							name: 'assets/[name].[ext]',
						}
					}
				]
			},
			{
				test: /\.(woff|woff2|eot|ttf|otf)$/,
				loader: 'file-loader',
				options: {
					name: 'assets/[name].[ext]'
				}
			}
		]
	},
	devServer: {
		contentBase: path.join(__dirname, 'src'),
		compress: true,
		port: 8080
	},
	plugins: [
		new HtmlWebpackPlugin({
			template: 'src/index.html'
		}),
    new HotModuleReplacementPlugin(),
    new CleanWebpackPlugin()
	],
	resolve: {
		extensions: ['.js']
	}
};
