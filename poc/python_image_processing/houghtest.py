import cv2 as cv
import numpy as np


if __name__ == '__main__':
    img = cv.imread("images\\rectified.png")
    gray = cv.cvtColor(img, cv.COLOR_BGR2GRAY)
    edges = cv.Canny(gray, 50, 200, apertureSize=3)
    lines = cv.HoughLinesP(edges, 1, np.pi / 180, 100, 0, 0)

    cv.imshow("hough test", edges)
    cv.waitKey(0)
    cv.destroyAllWindows()