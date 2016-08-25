import React, { Component } from 'react';
import './App.css';
import marked from 'marked';

class App extends Component {
  constructor() {
    super();
    this.state = {
      isLoading: false,
      error: "",
      repository: "",
      changelog: ""
    };
  }

  handleSubmit(e) {
    e.preventDefault();
    this.setState({
      error: "",
      isLoading: true
    });

    const xhr = new XMLHttpRequest();
    xhr.addEventListener('load', () => {
      this.setState(JSON.parse(xhr.responseText));
      this.setState({ isLoading: false });
    });
    xhr.open('POST', '/generate');
    xhr.setRequestHeader('Content-Type', 'application/json;charset=UTF-8');
    xhr.send(JSON.stringify({
      repository: this.state.repository
    }));
  }

  getContent() {
    return {
      __html: marked(this.state.changelog)
    };
  }

  render() {
    const { isLoading, error, repository } = this.state;

    const errorMessage = !error ? null : (
      <div className="wrapper--centered error-box state--error">
        { error }
      </div>
    );

    const loadingMessage = !isLoading ? null : (
      <div className="loader">Loading...</div>
    );

    return (
      <div className="site-wrapper">
        <h1 className="clog-logo">clog</h1>
        <form className="wrapper--centered generate-form" onSubmit={e => this.handleSubmit(e)}>
          <input 
            className="text-box--big"
            value={repository}
            type="input"
            name="repository"
            disabled={isLoading}
            autoFocus={true}
            onChange={e => this.setState({ repository: e.target.value })}
          />
          <button className="generate-form__button">&#9881;</button>
        </form>

        { loadingMessage }
        { errorMessage }

        <div className="wrapper--centered" dangerouslySetInnerHTML={this.getContent()}></div>
      </div>
    );
  }
}

export default App;
