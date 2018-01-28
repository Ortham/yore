import React from 'react';
import renderer from 'react-test-renderer';

jest.mock('../../src/gui/js/main-panel', () => 'MainPanel');
jest.mock('../../src/gui/js/sidebar', () => 'Sidebar');
jest.unmock('../../src/gui/js/requests');

import * as requests from '../../src/gui/js/requests'; // eslint-disable-line import/first
import Page from '../../src/gui/js/page'; // eslint-disable-line import/first

describe('Page', () => {
  let page;

  beforeAll(() => {
    const photos = [
      {
        height: 5,
        width: 10,
        loaded: false,
        path: 'path 1',
        src: 'source 1'
      },
      {
        height: 5,
        width: 10,
        loaded: true,
        path: 'path 2',
        src: 'source 2',
        location: {
          Suggested: [
            {
              latitude: 52.0,
              longitude: 13.2
            },
            {
              meters: 5,
              seconds: 20
            }
          ]
        }
      }
    ];

    requests.writeCoordinates = jest
      .fn()
      .mockReturnValueOnce(Promise.resolve());
    requests.getFilteredPhotos = jest
      .fn()
      .mockReturnValueOnce(Promise.resolve([photos[1]]));
    requests.getLocation = jest.fn().mockReturnValue(Promise.resolve());
    requests.getLocations = jest.fn().mockReturnValue(
      Promise.resolve([
        { error: 'Oh no!' },
        {
          location: {
            Existing: {
              latitude: 5,
              longitude: 30
            }
          }
        }
      ])
    );

    page = renderer.create(<Page rootPath="" photos={photos} />, {
      createNodeMock: element => {
        if (element.type === 'Sidebar') {
          return {
            forceUpdate: jest.fn()
          };
        }
        return null;
      }
    });
  });

  beforeEach(() => {
    requests.writeCoordinates.mockClear();
    requests.getFilteredPhotos.mockClear();
    requests.getLocation.mockClear();
    requests.getLocations.mockClear();
  });

  test('renders a header, sidebar and main panel', () => {
    expect(page.toJSON()).toMatchSnapshot();
  });

  test('handlePhotoSelect should set the current photo to the given photo', () => {
    const pageInstance = page.root.instance;

    expect(pageInstance.state.currentPhoto).toBe(undefined);

    pageInstance.handlePhotoSelect(pageInstance.state.photos[1]);

    expect(pageInstance.state.currentPhoto).toEqual(
      pageInstance.state.photos[1]
    );
    expect(pageInstance.sidebar.forceUpdate.mock.calls.length).toBe(1);
  });

  test('handleSuggestionApply calls writeCoordinates then moves suggested location to existing', () => {
    const pageInstance = page.root.instance;

    pageInstance.handlePhotoSelect(pageInstance.state.photos[1]);

    return pageInstance.handleSuggestionApply().then(() => {
      expect(requests.writeCoordinates.mock.calls.length).toBe(1);
      expect(pageInstance.state.currentPhoto.location).toEqual({
        Existing: {
          latitude: 52.0,
          longitude: 13.2
        }
      });
      expect(pageInstance.state.currentPhoto).toEqual(
        pageInstance.state.photos[1]
      );
      expect(pageInstance.sidebar.forceUpdate.mock.calls.length).toBe(1);
    });
  });

  test("handleSuggestionDiscard should set the current photo's location to be undefined", () => {
    const pageInstance = page.root.instance;

    pageInstance.handlePhotoSelect(pageInstance.state.photos[1]);

    pageInstance.handleSuggestionDiscard();

    expect(pageInstance.state.currentPhoto.location).toBe(undefined);
    expect(pageInstance.state.currentPhoto).toEqual(
      pageInstance.state.photos[1]
    );
    expect(pageInstance.sidebar.forceUpdate.mock.calls.length).toBe(1);
  });

  test('handleFilterToggle should call getFilteredPhotos if the filter is enabled', () => {
    const pageInstance = page.root.instance;

    expect(pageInstance.state.filterPhotos).toBe(false);

    return pageInstance
      .handleFilterToggle({ target: { checked: true } })
      .then(() => {
        expect(requests.getFilteredPhotos.mock.calls.length).toBe(1);
        expect(pageInstance.state.filterPhotos).toBe(true);
        expect(pageInstance.state.sidebarPhotos.length).toBe(1);
        expect(pageInstance.state.sidebarPhotos[0]).toEqual(
          pageInstance.state.photos[1]
        );
      });
  });

  test('handleFilterToggle should call set full photos array if the filter is disabled', () => {
    const pageInstance = page.root.instance;

    return pageInstance
      .handleFilterToggle({ target: { checked: false } })
      .then(() => {
        expect(requests.getFilteredPhotos.mock.calls.length).toBe(0);
        expect(pageInstance.state.filterPhotos).toBe(false);
        expect(pageInstance.state.sidebarPhotos.length).toBe(2);
        expect(pageInstance.state.sidebarPhotos).toEqual(
          pageInstance.state.photos
        );
      });
  });

  test('getLocationsPromise calls getLocation for each photo if filterPhotos is true', () => {
    const pageInstance = page.root.instance;

    pageInstance.setState(
      Object.assign({}, pageInstance.state, { filterPhotos: true })
    );

    return pageInstance.getLocationsPromise(0, 2).then(() => {
      expect(requests.getLocation.mock.calls.length).toBe(2);
    });
  });

  test('getLocationsPromise calls getLocations for range if filterPhotos is false', () => {
    const pageInstance = page.root.instance;

    pageInstance.setState(
      Object.assign({}, pageInstance.state, { filterPhotos: false })
    );

    return pageInstance.getLocationsPromise(0, 2).then(() => {
      expect(requests.getLocations.mock.calls.length).toBe(1);
    });
  });

  test('getAndStoreLocations sets location, error and loaded photo fields', () => {
    const pageInstance = page.root.instance;

    return pageInstance.getAndStoreLocations(0, 2).then(() => {
      expect(requests.getLocations.mock.calls.length).toBe(1);

      expect(pageInstance.state.sidebarPhotos[0].location).toBe(undefined);
      expect(pageInstance.state.sidebarPhotos[0].error).toBe('Oh no!');
      expect(pageInstance.state.sidebarPhotos[0].loaded).toBe(true);
      expect(pageInstance.state.sidebarPhotos[1].location).toEqual({
        Existing: {
          latitude: 5,
          longitude: 30
        }
      });
      expect(pageInstance.state.sidebarPhotos[1].error).toBe(undefined);
      expect(pageInstance.state.sidebarPhotos[1].loaded).toBe(true);
    });
  });
});
