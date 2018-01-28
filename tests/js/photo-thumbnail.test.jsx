import React from 'react';
import renderer from 'react-test-renderer';
import PhotoThumbnail from '../../src/gui/js/photo-thumbnail';

describe('PhotoThumbnail', () => {
  let mockHandleSelect;
  let thumbnail;

  beforeAll(() => {
    mockHandleSelect = jest.fn();

    thumbnail = renderer
      .create(
        <PhotoThumbnail
          key={0}
          style={{ top: 5 }}
          isSelected={false}
          handleSelect={mockHandleSelect}
          photo={{
            path: 'path',
            src: 'source'
          }}
        />
      )
      .toJSON();
  });

  beforeEach(() => {
    mockHandleSelect.mockClear();
  });

  test('renders a div with an image and an icon', () => {
    expect(thumbnail).toMatchSnapshot();
  });

  test('clicking on a PhotoThumbnail should call its handleSelect callback', () => {
    thumbnail.props.onClick();
    expect(mockHandleSelect.mock.calls.length).toBe(1);
  });

  test('a key-up on a PhotoThumbnail should call its handleSelect callback', () => {
    thumbnail.props.onKeyUp();
    expect(mockHandleSelect.mock.calls.length).toBe(1);
  });
});
