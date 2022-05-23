// sample.h
#include <vector> // #include directive
#include <string>
using namespace std;


    class Constructor{
   
    public:
    //todo does this return matrix correctly?
    //todo add curvemap to signature
        int constructMap(int rastersize, int  contourMargin);
        
    private: 
        int rasterSize;
        int contourMargin;

        int xCells;
        int yCells;

        int xDimensions;
        int yDimensions;

        vector<vector<vector<float>>> contourLines;

        vector<vector<bool>> isSCV;
        vector<vector<float>> knownHeights;

        void initialize();
        bool checkSVC(vector<float> p, int  ix, int iy );
        float localTIN(vector<float> p);
        bool calcheigtsNVC();
    };

