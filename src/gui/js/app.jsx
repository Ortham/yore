import React from 'react';
import ReactDOM from 'react-dom';
import Page from './page';

function getInitialState() {
  const state = {
    rootPath: undefined,
    filterPhotos: false,
    currentPhoto: undefined,
    photos: []
  };

  return fetch('/rootPath')
    .then(response => response.json())
    .then(responseBody => {
      state.rootPath = responseBody.rootPath;

      return fetch('/photos');
    })
    .then(response => response.json())
    .then(responseBody => {
      state.photos = responseBody.photos.map(photo =>
        Object.assign(photo, {
          src: `/thumbnail?path=${encodeURIComponent(
            photo.path
          )}&maxWidth=500&maxHeight=500`
        })
      );

      return state;
    });
}

function getFilteredPhotos(photos) {
  return fetch('/photos?filter')
    .then(response => response.json())
    .then(responseBody =>
      responseBody.photo_indices.map(index =>
        Object.assign({}, photos[index], { index })
      )
    );
}

function writeCoordinates(path, coordinates) {
  const url = `/location?path=${encodeURIComponent(path)}`;
  const init = {
    method: 'PUT',
    body: JSON.stringify(coordinates)
  };

  return fetch(url, init).then(response => {
    if (!response.ok) {
      throw new Error(`Failed to ${init.method} ${url} with body ${init.body}`);
    }
  });
}

function getLocations(startIndex, endIndex) {
  const url = `/locations?start=${startIndex}&end=${endIndex}`;

  return fetch(url)
    .then(response => {
      if (response.ok) {
        return response.json();
      }
      throw new Error(`Failed to GET ${url}`);
    })
    .then(responseBody => responseBody.locations);
}

function getLocation(path) {
  const url = `/location?path=${encodeURIComponent(path)}`;

  return fetch(url).then(response => {
    if (response.ok) {
      return response.json();
    }
    throw new Error(`Failed to GET ${url}`);
  });
}

getInitialState().then(state => {
  ReactDOM.render(
    <Page
      rootPath={state.rootPath}
      photos={state.photos}
      writeCoordinates={writeCoordinates}
      getLocations={getLocations}
      getLocation={getLocation}
      getFilteredPhotos={getFilteredPhotos}
    />,
    document.getElementById('root')
  );
});
