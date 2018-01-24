function get(url) {
  return fetch(url).then(response => {
    if (response.ok) {
      return response.json();
    }
    throw new Error(`Failed to GET ${url}`);
  });
}

function put(url, body) {
  const init = {
    method: 'PUT',
    body: JSON.stringify(body)
  };

  return fetch(url, init).then(response => {
    if (!response.ok) {
      throw new Error(`Failed to ${init.method} ${url} with body ${init.body}`);
    }
  });
}

export function getRootPath() {
  return get('/rootPath');
}

export function getPhotos() {
  return get('/photos').then(responseBody =>
    responseBody.photos.map(photo =>
      Object.assign(photo, {
        src: `/thumbnail?path=${encodeURIComponent(
          photo.path
        )}&maxWidth=500&maxHeight=500`
      })
    )
  );
}

export function getFilteredPhotos(photos) {
  return get('/photos?filter').then(responseBody =>
    responseBody.photo_indices.map(index =>
      Object.assign({}, photos[index], { index })
    )
  );
}

export function writeCoordinates(path, coordinates) {
  return put(`/location?path=${encodeURIComponent(path)}`, coordinates);
}

export function getLocations(startIndex, endIndex) {
  return get(`/locations?start=${startIndex}&end=${endIndex}`).then(
    responseBody => responseBody.locations
  );
}

export function getLocation(path) {
  return get(`/location?path=${encodeURIComponent(path)}`);
}
