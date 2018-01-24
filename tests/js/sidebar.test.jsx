import React from 'react';
import renderer from 'react-test-renderer';
import Sidebar from '../../src/gui/js/sidebar';

describe('Sidebar', () => {
  let mockGetAndStoreLocations;
  let mockHandleFilterToggle;
  let mockHandlePhotoSelect;
  let filteredSidebar;

  const photos = [
    {
      height: 5,
      width: 10,
      loaded: false,
      path: 'path',
      src: 'source'
    },
    {
      height: 5,
      width: 10,
      loaded: true,
      path: 'path',
      src: 'source'
    }
  ];

  beforeAll(() => {
    mockGetAndStoreLocations = jest.fn();
    mockHandleFilterToggle = jest.fn().mockReturnValueOnce(Promise.resolve());
    mockHandlePhotoSelect = jest.fn();

    filteredSidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    );
  });

  beforeEach(() => {
    mockGetAndStoreLocations.mockClear();
    mockHandleFilterToggle.mockClear();
    mockHandlePhotoSelect.mockClear();
  });

  test('renders a list of photo thumbnails and a filter checkbox and label', () => {
    const sidebar = renderer
      .create(
        <Sidebar
          photos={photos}
          filterPhotos={false}
          getAndStoreLocations={mockGetAndStoreLocations}
          handleFilterToggle={mockHandleFilterToggle}
          handlePhotoSelect={mockHandlePhotoSelect}
        />
      )
      .toJSON();
    expect(sidebar).toMatchSnapshot();
  });

  test('passing filterPhotos as true should check the filter checkbox', () => {
    expect(filteredSidebar.toJSON()).toMatchSnapshot();
  });

  test('rowRenderer returns a PhotoThumbnail for the photo at the given index', () => {
    const sidebar = filteredSidebar.root.instance;

    const photoThumbnail = sidebar.rowRenderer({
      index: 0,
      key: 1,
      style: { color: 'black' }
    });
    expect(photoThumbnail).toMatchSnapshot();
  });

  test('rowRender should set the PhotoThumbnail handleSelect to call the handlePhotoSelect callback', () => {
    const sidebar = filteredSidebar.root.instance;

    const photoThumbnail = sidebar.rowRenderer({
      index: 0,
      key: 1,
      style: { color: 'black' }
    });

    photoThumbnail.props.handleSelect();

    expect(mockHandlePhotoSelect.mock.calls.length).toBe(1);
    expect(mockHandlePhotoSelect.mock.calls[0].length).toBe(1);
    expect(mockHandlePhotoSelect.mock.calls[0][0]).toBe(photos[0]);
  });

  test('rowHeight scales the height of the photo at the given index to match a width of 272', () => {
    const sidebar = filteredSidebar.root.instance;

    expect(sidebar.rowHeight({ index: 0 })).toBe(136);
  });

  test('isRowLoaded returns false if the photo at the given index has loaded = false', () => {
    const sidebar = filteredSidebar.root.instance;

    expect(sidebar.isRowLoaded({ index: 0 })).toBe(false);
  });

  test('isRowLoaded returns true if the photo at the given index has loaded = true', () => {
    const sidebar = filteredSidebar.root.instance;

    expect(sidebar.isRowLoaded({ index: 1 })).toBe(true);
  });

  test('loadMoreRows should call the getAndStoreLocations callback', () => {
    const sidebar = filteredSidebar.root.instance;

    sidebar.loadMoreRows({ startIndex: 0, stopIndex: 1 });
    expect(mockGetAndStoreLocations.mock.calls.length).toBe(1);
    expect(mockGetAndStoreLocations.mock.calls[0]).toEqual([0, 1]);
  });

  test('handleFilterToggle should call the handleFilterToggle callback', () => {
    const sidebar = filteredSidebar.root.instance;
    const event = { target: { checked: true } };

    sidebar.list.recomputeRowHeights = jest.fn();

    return sidebar.handleFilterToggle(event).then(() => {
      expect(mockHandleFilterToggle.mock.calls.length).toBe(1);
      expect(mockHandleFilterToggle.mock.calls[0].length).toBe(1);
      expect(mockHandleFilterToggle.mock.calls[0][0]).toBe(event);
      expect(sidebar.list.recomputeRowHeights.mock.calls.length).toBe(1);
    });
  });

  test("forceUpdate should force the list's grid to update and reset the loaded rows cache", () => {
    const sidebar = filteredSidebar.root.instance;

    sidebar.list.forceUpdateGrid = jest.fn();
    sidebar.loader.resetLoadMoreRowsCache = jest.fn();

    sidebar.forceUpdate();

    expect(sidebar.list.forceUpdateGrid.mock.calls.length).toBe(1);
    expect(sidebar.loader.resetLoadMoreRowsCache.mock.calls.length).toBe(1);
    expect(sidebar.loader.resetLoadMoreRowsCache.mock.calls[0]).toEqual([true]);
  });
});
