mod model_construction;

use model_construction::level_curve_tree::*;
use model_construction::level_curves::*;
use model_construction::point::*;
use model_construction::raster::Raster;
use model_construction::constructor::ModelConstructor;

use std::fs::File;
use std::io::Write;


pub fn main() {

    let raster_size = 850.0;
    let row_no: usize = 30;
    let col_no: usize = 30;
    let contour_margin = 18000.0;
    let file_name = "output.obj";


    let mut level_curve_0: LevelCurve = LevelCurve::new(50.0);
    let mut level_curve_1: LevelCurve = LevelCurve::new(100.0);
    let mut level_curve_2: LevelCurve = LevelCurve::new(150.0);
    let mut level_curve_3: LevelCurve = LevelCurve::new(200.0);
    let mut level_curve_4: LevelCurve = LevelCurve::new(250.0);

    // Add all points to level curve 0
    level_curve_0.add_all_points(
        vec![Point{x:798.0,y:101.0,z:0.0}, Point{x:789.0,y:92.0,z:0.0}, Point{x:777.0,y:84.0,z:0.0}, Point{x:753.0,y:71.0,z:0.0}, Point{x:743.0,y:67.0,z:0.0}, Point{x:730.0,y:65.0,z:0.0}, Point{x:717.0,y:63.0,z:0.0}, Point{x:703.0,y:63.0,z:0.0}, Point{x:691.0,y:63.0,z:0.0}, Point{x:678.0,y:66.0,z:0.0}, Point{x:665.0,y:70.0,z:0.0}, Point{x:651.0,y:77.0,z:0.0}, Point{x:638.0,y:85.0,z:0.0}, Point{x:627.0,y:93.0,z:0.0}, Point{x:618.0,y:102.0,z:0.0}, Point{x:613.0,y:111.0,z:0.0}, Point{x:607.0,y:120.0,z:0.0}, Point{x:599.0,y:136.0,z:0.0}, Point{x:594.0,y:148.0,z:0.0}, Point{x:589.0,y:157.0,z:0.0}, Point{x:583.0,y:170.0,z:0.0}, Point{x:577.0,y:184.0,z:0.0}, Point{x:573.0,y:194.0,z:0.0}, Point{x:568.0,y:209.0,z:0.0}, Point{x:563.0,y:221.0,z:0.0}, Point{x:560.0,y:234.0,z:0.0}, Point{x:554.0,y:246.0,z:0.0}, Point{x:550.0,y:259.0,z:0.0}, Point{x:544.0,y:272.0,z:0.0}, Point{x:536.0,y:285.0,z:0.0}, Point{x:528.0,y:297.0,z:0.0}, Point{x:520.0,y:310.0,z:0.0}, Point{x:511.0,y:323.0,z:0.0}, Point{x:504.0,y:331.0,z:0.0}, Point{x:493.0,y:343.0,z:0.0}, Point{x:485.0,y:350.0,z:0.0}, Point{x:470.0,y:362.0,z:0.0}, Point{x:456.0,y:374.0,z:0.0}, Point{x:441.0,y:384.0,z:0.0}, Point{x:425.0,y:394.0,z:0.0}, Point{x:409.0,y:404.0,z:0.0}, Point{x:398.0,y:409.0,z:0.0}, Point{x:383.0,y:417.0,z:0.0}, Point{x:368.0,y:424.0,z:0.0}, Point{x:350.0,y:433.0,z:0.0}, Point{x:333.0,y:439.0,z:0.0}, Point{x:314.0,y:446.0,z:0.0}, Point{x:295.0,y:453.0,z:0.0}, Point{x:278.0,y:461.0,z:0.0}, Point{x:262.0,y:467.0,z:0.0}, Point{x:247.0,y:474.0,z:0.0}, Point{x:236.0,y:479.0,z:0.0}, Point{x:221.0,y:488.0,z:0.0}, Point{x:210.0,y:494.0,z:0.0}, Point{x:195.0,y:504.0,z:0.0}, Point{x:186.0,y:509.0,z:0.0}, Point{x:174.0,y:519.0,z:0.0}, Point{x:158.0,y:533.0,z:0.0}, Point{x:150.0,y:541.0,z:0.0}, Point{x:141.0,y:552.0,z:0.0}, Point{x:134.0,y:562.0,z:0.0}, Point{x:128.0,y:574.0,z:0.0}, Point{x:122.0,y:585.0,z:0.0}, Point{x:117.0,y:596.0,z:0.0}, Point{x:113.0,y:606.0,z:0.0}, Point{x:111.0,y:617.0,z:0.0}, Point{x:111.0,y:630.0,z:0.0}, Point{x:114.0,y:640.0,z:0.0}, Point{x:117.0,y:651.0,z:0.0}, Point{x:122.0,y:661.0,z:0.0}, Point{x:129.0,y:671.0,z:0.0}, Point{x:136.0,y:680.0,z:0.0}, Point{x:145.0,y:689.0,z:0.0}, Point{x:154.0,y:696.0,z:0.0}, Point{x:164.0,y:703.0,z:0.0}, Point{x:173.0,y:708.0,z:0.0}, Point{x:187.0,y:715.0,z:0.0}, Point{x:204.0,y:720.0,z:0.0}, Point{x:225.0,y:724.0,z:0.0}, Point{x:250.0,y:727.0,z:0.0}, Point{x:271.0,y:729.0,z:0.0}, Point{x:294.0,y:732.0,z:0.0}, Point{x:309.0,y:732.0,z:0.0}, Point{x:333.0,y:734.0,z:0.0}, Point{x:359.0,y:734.0,z:0.0}, Point{x:383.0,y:735.0,z:0.0}, Point{x:406.0,y:736.0,z:0.0}, Point{x:428.0,y:736.0,z:0.0}, Point{x:449.0,y:737.0,z:0.0}, Point{x:467.0,y:737.0,z:0.0}, Point{x:484.0,y:739.0,z:0.0}, Point{x:502.0,y:740.0,z:0.0}, Point{x:520.0,y:741.0,z:0.0}, Point{x:542.0,y:742.0,z:0.0}, Point{x:563.0,y:744.0,z:0.0}, Point{x:592.0,y:747.0,z:0.0}, Point{x:607.0,y:748.0,z:0.0}, Point{x:620.0,y:749.0,z:0.0}, Point{x:636.0,y:751.0,z:0.0}, Point{x:652.0,y:753.0,z:0.0}, Point{x:680.0,y:755.0,z:0.0}, Point{x:691.0,y:755.0,z:0.0}, Point{x:709.0,y:757.0,z:0.0}, Point{x:738.0,y:759.0,z:0.0}, Point{x:765.0,y:759.0,z:0.0}, Point{x:790.0,y:759.0,z:0.0}, Point{x:815.0,y:757.0,z:0.0}, Point{x:835.0,y:754.0,z:0.0}, Point{x:846.0,y:753.0,z:0.0}, Point{x:866.0,y:748.0,z:0.0}, Point{x:878.0,y:744.0,z:0.0}, Point{x:888.0,y:738.0,z:0.0}, Point{x:898.0,y:731.0,z:0.0}, Point{x:911.0,y:718.0,z:0.0}, Point{x:918.0,y:710.0,z:0.0}, Point{x:926.0,y:700.0,z:0.0}, Point{x:936.0,y:684.0,z:0.0}, Point{x:945.0,y:667.0,z:0.0}, Point{x:949.0,y:654.0,z:0.0}, Point{x:953.0,y:643.0,z:0.0}, Point{x:955.0,y:633.0,z:0.0}, Point{x:957.0,y:620.0,z:0.0}, Point{x:959.0,y:609.0,z:0.0}, Point{x:960.0,y:594.0,z:0.0}, Point{x:961.0,y:582.0,z:0.0}, Point{x:962.0,y:571.0,z:0.0}, Point{x:962.0,y:558.0,z:0.0}, Point{x:959.0,y:545.0,z:0.0}, Point{x:957.0,y:535.0,z:0.0}, Point{x:953.0,y:523.0,z:0.0}, Point{x:948.0,y:509.0,z:0.0}, Point{x:943.0,y:495.0,z:0.0}, Point{x:938.0,y:479.0,z:0.0}, Point{x:934.0,y:468.0,z:0.0}, Point{x:931.0,y:457.0,z:0.0}, Point{x:926.0,y:445.0,z:0.0}, Point{x:921.0,y:431.0,z:0.0}, Point{x:917.0,y:414.0,z:0.0}, Point{x:912.0,y:400.0,z:0.0}, Point{x:905.0,y:386.0,z:0.0}, Point{x:901.0,y:375.0,z:0.0}, Point{x:896.0,y:360.0,z:0.0}, Point{x:893.0,y:348.0,z:0.0}, Point{x:890.0,y:338.0,z:0.0}, Point{x:886.0,y:325.0,z:0.0}, Point{x:881.0,y:315.0,z:0.0}, Point{x:879.0,y:305.0,z:0.0}, Point{x:874.0,y:292.0,z:0.0}, Point{x:871.0,y:281.0,z:0.0}, Point{x:867.0,y:267.0,z:0.0}, Point{x:864.0,y:253.0,z:0.0}, Point{x:861.0,y:239.0,z:0.0}, Point{x:858.0,y:228.0,z:0.0}, Point{x:855.0,y:215.0,z:0.0}, Point{x:850.0,y:203.0,z:0.0}, Point{x:846.0,y:191.0,z:0.0}, Point{x:841.0,y:181.0,z:0.0}, Point{x:836.0,y:170.0,z:0.0}, Point{x:830.0,y:161.0,z:0.0}, Point{x:824.0,y:151.0,z:0.0}, Point{x:819.0,y:142.0,z:0.0}]
    );

    // Add all points to level curve 1
    level_curve_1.add_all_points(
        vec![Point{x:768.0,y:324.0,z:0.0}, Point{x:760.0,y:316.0,z:0.0}, Point{x:741.0,y:313.0,z:0.0}, Point{x:723.0,y:312.0,z:0.0}, Point{x:712.0,y:314.0,z:0.0}, Point{x:697.0,y:317.0,z:0.0}, Point{x:684.0,y:322.0,z:0.0}, Point{x:675.0,y:327.0,z:0.0}, Point{x:664.0,y:334.0,z:0.0}, Point{x:656.0,y:342.0,z:0.0}, Point{x:649.0,y:350.0,z:0.0}, Point{x:642.0,y:358.0,z:0.0}, Point{x:634.0,y:367.0,z:0.0}, Point{x:625.0,y:376.0,z:0.0}, Point{x:618.0,y:384.0,z:0.0}, Point{x:611.0,y:393.0,z:0.0}, Point{x:602.0,y:401.0,z:0.0}, Point{x:593.0,y:409.0,z:0.0}, Point{x:577.0,y:423.0,z:0.0}, Point{x:565.0,y:432.0,z:0.0}, Point{x:552.0,y:440.0,z:0.0}, Point{x:538.0,y:448.0,z:0.0}, Point{x:523.0,y:456.0,z:0.0}, Point{x:511.0,y:461.0,z:0.0}, Point{x:497.0,y:467.0,z:0.0}, Point{x:485.0,y:471.0,z:0.0}, Point{x:466.0,y:478.0,z:0.0}, Point{x:455.0,y:481.0,z:0.0}, Point{x:437.0,y:487.0,z:0.0}, Point{x:418.0,y:493.0,z:0.0}, Point{x:400.0,y:499.0,z:0.0}, Point{x:383.0,y:505.0,z:0.0}, Point{x:368.0,y:511.0,z:0.0}, Point{x:357.0,y:516.0,z:0.0}, Point{x:346.0,y:521.0,z:0.0}, Point{x:330.0,y:528.0,z:0.0}, Point{x:316.0,y:535.0,z:0.0}, Point{x:307.0,y:540.0,z:0.0}, Point{x:295.0,y:545.0,z:0.0}, Point{x:282.0,y:554.0,z:0.0}, Point{x:270.0,y:563.0,z:0.0}, Point{x:256.0,y:574.0,z:0.0}, Point{x:248.0,y:581.0,z:0.0}, Point{x:236.0,y:595.0,z:0.0}, Point{x:231.0,y:604.0,z:0.0}, Point{x:229.0,y:616.0,z:0.0}, Point{x:232.0,y:629.0,z:0.0}, Point{x:239.0,y:639.0,z:0.0}, Point{x:250.0,y:647.0,z:0.0}, Point{x:264.0,y:653.0,z:0.0}, Point{x:274.0,y:656.0,z:0.0}, Point{x:285.0,y:658.0,z:0.0}, Point{x:297.0,y:660.0,z:0.0}, Point{x:309.0,y:662.0,z:0.0}, Point{x:324.0,y:663.0,z:0.0}, Point{x:338.0,y:665.0,z:0.0}, Point{x:353.0,y:666.0,z:0.0}, Point{x:366.0,y:668.0,z:0.0}, Point{x:376.0,y:669.0,z:0.0}, Point{x:390.0,y:670.0,z:0.0}, Point{x:405.0,y:672.0,z:0.0}, Point{x:421.0,y:673.0,z:0.0}, Point{x:434.0,y:674.0,z:0.0}, Point{x:449.0,y:675.0,z:0.0}, Point{x:467.0,y:677.0,z:0.0}, Point{x:481.0,y:677.0,z:0.0}, Point{x:494.0,y:679.0,z:0.0}, Point{x:509.0,y:681.0,z:0.0}, Point{x:519.0,y:682.0,z:0.0}, Point{x:533.0,y:683.0,z:0.0}, Point{x:546.0,y:684.0,z:0.0}, Point{x:558.0,y:684.0,z:0.0}, Point{x:570.0,y:684.0,z:0.0}, Point{x:586.0,y:684.0,z:0.0}, Point{x:601.0,y:684.0,z:0.0}, Point{x:617.0,y:685.0,z:0.0}, Point{x:632.0,y:686.0,z:0.0}, Point{x:652.0,y:688.0,z:0.0}, Point{x:666.0,y:691.0,z:0.0}, Point{x:681.0,y:692.0,z:0.0}, Point{x:694.0,y:694.0,z:0.0}, Point{x:704.0,y:696.0,z:0.0}, Point{x:718.0,y:698.0,z:0.0}, Point{x:732.0,y:700.0,z:0.0}, Point{x:746.0,y:702.0,z:0.0}, Point{x:758.0,y:704.0,z:0.0}, Point{x:769.0,y:706.0,z:0.0}, Point{x:781.0,y:707.0,z:0.0}, Point{x:793.0,y:708.0,z:0.0}, Point{x:805.0,y:709.0,z:0.0}, Point{x:815.0,y:710.0,z:0.0}, Point{x:829.0,y:710.0,z:0.0}, Point{x:843.0,y:707.0,z:0.0}, Point{x:853.0,y:704.0,z:0.0}, Point{x:863.0,y:700.0,z:0.0}, Point{x:876.0,y:694.0,z:0.0}, Point{x:886.0,y:688.0,z:0.0}, Point{x:899.0,y:680.0,z:0.0}, Point{x:910.0,y:669.0,z:0.0}, Point{x:917.0,y:659.0,z:0.0}, Point{x:924.0,y:649.0,z:0.0}, Point{x:928.0,y:637.0,z:0.0}, Point{x:931.0,y:625.0,z:0.0}, Point{x:932.0,y:614.0,z:0.0}, Point{x:934.0,y:603.0,z:0.0}, Point{x:936.0,y:589.0,z:0.0}, Point{x:936.0,y:577.0,z:0.0}, Point{x:934.0,y:563.0,z:0.0}, Point{x:931.0,y:552.0,z:0.0}, Point{x:927.0,y:542.0,z:0.0}, Point{x:921.0,y:529.0,z:0.0}, Point{x:916.0,y:518.0,z:0.0}, Point{x:909.0,y:507.0,z:0.0}, Point{x:901.0,y:495.0,z:0.0}, Point{x:894.0,y:485.0,z:0.0}, Point{x:887.0,y:476.0,z:0.0}, Point{x:878.0,y:467.0,z:0.0}, Point{x:867.0,y:455.0,z:0.0}, Point{x:857.0,y:445.0,z:0.0}, Point{x:847.0,y:433.0,z:0.0}, Point{x:836.0,y:421.0,z:0.0}, Point{x:824.0,y:411.0,z:0.0}, Point{x:817.0,y:403.0,z:0.0}, Point{x:809.0,y:390.0,z:0.0}, Point{x:804.0,y:381.0,z:0.0}, Point{x:798.0,y:370.0,z:0.0}, Point{x:793.0,y:361.0,z:0.0}, Point{x:789.0,y:350.0,z:0.0}, Point{x:785.0,y:340.0,z:0.0}]
    );

    // Add all points to level curve 1
    level_curve_2.add_all_points(
        vec![Point{x:744.0,y:459.0,z:0.0}, Point{x:734.0,y:450.0,z:0.0}, Point{x:722.0,y:447.0,z:0.0}, Point{x:706.0,y:447.0,z:0.0}, Point{x:691.0,y:450.0,z:0.0}, Point{x:678.0,y:454.0,z:0.0}, Point{x:667.0,y:459.0,z:0.0}, Point{x:658.0,y:466.0,z:0.0}, Point{x:649.0,y:475.0,z:0.0}, Point{x:641.0,y:484.0,z:0.0}, Point{x:634.0,y:492.0,z:0.0}, Point{x:624.0,y:504.0,z:0.0}, Point{x:614.0,y:511.0,z:0.0}, Point{x:601.0,y:518.0,z:0.0}, Point{x:587.0,y:526.0,z:0.0}, Point{x:574.0,y:533.0,z:0.0}, Point{x:558.0,y:538.0,z:0.0}, Point{x:546.0,y:542.0,z:0.0}, Point{x:527.0,y:548.0,z:0.0}, Point{x:511.0,y:555.0,z:0.0}, Point{x:501.0,y:560.0,z:0.0}, Point{x:489.0,y:567.0,z:0.0}, Point{x:479.0,y:575.0,z:0.0}, Point{x:468.0,y:587.0,z:0.0}, Point{x:463.0,y:599.0,z:0.0}, Point{x:467.0,y:611.0,z:0.0}, Point{x:480.0,y:618.0,z:0.0}, Point{x:492.0,y:622.0,z:0.0}, Point{x:508.0,y:624.0,z:0.0}, Point{x:520.0,y:625.0,z:0.0}, Point{x:533.0,y:627.0,z:0.0}, Point{x:548.0,y:627.0,z:0.0}, Point{x:559.0,y:629.0,z:0.0}, Point{x:573.0,y:630.0,z:0.0}, Point{x:584.0,y:630.0,z:0.0}, Point{x:597.0,y:632.0,z:0.0}, Point{x:608.0,y:632.0,z:0.0}, Point{x:618.0,y:633.0,z:0.0}, Point{x:628.0,y:635.0,z:0.0}, Point{x:639.0,y:635.0,z:0.0}, Point{x:649.0,y:636.0,z:0.0}, Point{x:660.0,y:638.0,z:0.0}, Point{x:670.0,y:641.0,z:0.0}, Point{x:682.0,y:642.0,z:0.0}, Point{x:696.0,y:646.0,z:0.0}, Point{x:708.0,y:648.0,z:0.0}, Point{x:722.0,y:651.0,z:0.0}, Point{x:732.0,y:652.0,z:0.0}, Point{x:743.0,y:655.0,z:0.0}, Point{x:755.0,y:657.0,z:0.0}, Point{x:769.0,y:659.0,z:0.0}, Point{x:784.0,y:661.0,z:0.0}, Point{x:797.0,y:662.0,z:0.0}, Point{x:811.0,y:663.0,z:0.0}, Point{x:822.0,y:663.0,z:0.0}, Point{x:833.0,y:661.0,z:0.0}, Point{x:847.0,y:658.0,z:0.0}, Point{x:857.0,y:654.0,z:0.0}, Point{x:868.0,y:646.0,z:0.0}, Point{x:877.0,y:637.0,z:0.0}, Point{x:883.0,y:626.0,z:0.0}, Point{x:885.0,y:613.0,z:0.0}, Point{x:885.0,y:599.0,z:0.0}, Point{x:882.0,y:588.0,z:0.0}, Point{x:878.0,y:575.0,z:0.0}, Point{x:872.0,y:564.0,z:0.0}, Point{x:864.0,y:552.0,z:0.0}, Point{x:858.0,y:543.0,z:0.0}, Point{x:851.0,y:535.0,z:0.0}, Point{x:843.0,y:528.0,z:0.0}, Point{x:833.0,y:519.0,z:0.0}, Point{x:822.0,y:510.0,z:0.0}, Point{x:813.0,y:502.0,z:0.0}, Point{x:803.0,y:494.0,z:0.0}, Point{x:793.0,y:486.0,z:0.0}, Point{x:782.0,y:477.0,z:0.0}, Point{x:774.0,y:469.0,z:0.0}]
    );

    // Add all points to level curve 1
    level_curve_3.add_all_points(
        vec![Point{x:833.0,y:561.0,z:0.0}, Point{x:826.0,y:550.0,z:0.0}, Point{x:817.0,y:542.0,z:0.0}, Point{x:808.0,y:537.0,z:0.0}, Point{x:794.0,y:529.0,z:0.0}, Point{x:785.0,y:524.0,z:0.0}, Point{x:770.0,y:518.0,z:0.0}, Point{x:759.0,y:514.0,z:0.0}, Point{x:743.0,y:513.0,z:0.0}, Point{x:728.0,y:514.0,z:0.0}, Point{x:717.0,y:518.0,z:0.0}, Point{x:705.0,y:523.0,z:0.0}, Point{x:695.0,y:529.0,z:0.0}, Point{x:686.0,y:537.0,z:0.0}, Point{x:681.0,y:546.0,z:0.0}, Point{x:678.0,y:556.0,z:0.0}, Point{x:677.0,y:566.0,z:0.0}, Point{x:679.0,y:578.0,z:0.0}, Point{x:682.0,y:588.0,z:0.0}, Point{x:689.0,y:598.0,z:0.0}, Point{x:696.0,y:606.0,z:0.0}, Point{x:705.0,y:613.0,z:0.0}, Point{x:717.0,y:620.0,z:0.0}, Point{x:732.0,y:625.0,z:0.0}, Point{x:744.0,y:627.0,z:0.0}, Point{x:756.0,y:628.0,z:0.0}, Point{x:767.0,y:629.0,z:0.0}, Point{x:779.0,y:631.0,z:0.0}, Point{x:791.0,y:632.0,z:0.0}, Point{x:805.0,y:633.0,z:0.0}, Point{x:817.0,y:632.0,z:0.0}, Point{x:828.0,y:629.0,z:0.0}, Point{x:837.0,y:624.0,z:0.0}, Point{x:845.0,y:616.0,z:0.0}, Point{x:850.0,y:606.0,z:0.0}, Point{x:852.0,y:596.0,z:0.0}, Point{x:850.0,y:586.0,z:0.0}, Point{x:845.0,y:575.0,z:0.0}, Point{x:840.0,y:566.0,z:0.0}, Point{x:833.0,y:557.0,z:0.0}]
    );

    // Add all points to level curve 1
    level_curve_4.add_all_points(
        vec![Point{x:789.0,y:569.0,z:0.0}, Point{x:780.0,y:563.0,z:0.0}, Point{x:770.0,y:561.0,z:0.0}, Point{x:758.0,y:561.0,z:0.0}, Point{x:746.0,y:562.0,z:0.0}, Point{x:737.0,y:568.0,z:0.0}, Point{x:736.0,y:579.0,z:0.0}, Point{x:744.0,y:588.0,z:0.0}, Point{x:754.0,y:593.0,z:0.0}, Point{x:765.0,y:597.0,z:0.0}, Point{x:776.0,y:599.0,z:0.0}, Point{x:786.0,y:598.0,z:0.0}, Point{x:793.0,y:590.0,z:0.0}, Point{x:798.0,y:580.0,z:0.0}]
    );


    let mut level_curve_map: LevelCurveSet = LevelCurveSet::new(50.0);
    level_curve_map.add_level_curve(level_curve_0);
    level_curve_map.add_level_curve(level_curve_1);
    level_curve_map.add_level_curve(level_curve_2);
    level_curve_map.add_level_curve(level_curve_3);
    level_curve_map.add_level_curve(level_curve_4);

    
        //create raster based on given params
        
        //find max and min x and y in level curve model
        

        let mut raster = Raster::new(raster_size, raster_size, row_no, col_no );

        //create new modelConstructor (module containing 3D-model construction algorithm)
        let mut model_constructor = ModelConstructor::new(&mut raster, contour_margin, &level_curve_map);

        //determine heights
        print!("starting construction");
        model_constructor.construct();

        //get heights from raster
        let heights = &raster.altitudes;
   
        // println!("{}", x);
           
       let mut file = File::create(file_name).unwrap();
       let mut verts: String = String::new();
       let mut faces: String = String::new();
        let columns = col_no;
        let rows = col_no;


       //v x y z
       // f [list of indexes of vertices that are joined by face]
       let mut v = 1;
   
       for x in 0..raster.rows - 1{
           for y in 0 ..raster.columns -1 {
               let a = columns  as f32 * raster.column_width;

              // println!("{}", heights[x][y].unwrap() );
               //(0, 0)
               verts.push_str (&format!("v {a}.0 {b}.0 {c}.0 \n", a = (x  as f32  ) * raster.column_width ,
                                                                b = ((rows - y) as f32 ) * raster.row_height,
                                                                c = heights[x][y].unwrap()));
   
               //(0, 1)
               verts.push_str (&format!("v {a}.0 {b}.0 {c}.0 \n", a = (x as f32  ) * raster.column_width ,
                                                                       b = ((rows - y) as f32 + 1.0) * raster.row_height,
                                                                       c = heights[x][y + 1].unwrap()));
               //(1, 0)
               verts.push_str (&format!("v {a}.0 {b}.0 {c}.0 \n", a = (x as f32 + 1.0 ) * raster.column_width ,
                                                                       b = ((rows - y) as f32) * raster.row_height,
                                                                       c = heights[x + 1][y].unwrap()));
               //(1, 1)
               verts.push_str (&format!("v {a}.0 {b}.0 {c}.0 \n", a =( x as f32 + 1.0 ) * raster.column_width ,
                                                                           b = ((rows - y) as f32 + 1.0) * raster.row_height,
                                                                           c = heights[x+ 1][y+ 1].unwrap()));
   
               //t1 : (0,0) , (0, 1), (1, 1)
               faces.push_str (&format!("f {a} {b} {c} \n", a = v, b= v+1, c = v + 3 ));
               //t2 : (0,0) , (1, 0), (1, 1)
               faces.push_str (&format!("f {a} {b} {c} \n", a = v, b= v+2, c = v + 3 ));
   
               v = v + 4;
           } 
       }
   
       // for x in (1..10).step_by(2) {
       //     println!("{}", x);
       // }
   
       verts.push_str(&faces);
       file.write_all(verts.as_bytes());

}