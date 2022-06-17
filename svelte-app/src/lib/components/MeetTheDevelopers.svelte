<div class="meet_the_developers_wrapper">

    <h2>Meet the developers</h2>

    <div class="grid-container">
        <div class="grid-item image-container">
            <div class="profile-image adebruijn"></div>
        </div>
        <div class="grid-item text-container">
            <h4>Abel de Bruijn</h4>
            <p>Developer Frontend</p>
        </div>

        <div class="grid-item image-container">
            <div class="profile-image rdur"></div>
        </div>
        <div class="grid-item text-container">
            <h4>Rens Dur</h4>
            <p>Developer Rust and Frontend</p>
        </div>

        <div class="grid-item image-container">
            <div class="profile-image phengst"></div>
        </div>
        <div class="grid-item text-container">
            <h4>Pauline Hengst</h4>
            <p>Developer Rust and Graphics</p>
        </div>

        <div class="grid-item image-container">
            <div class="profile-image jvanderkris"></div>
        </div>
        <div class="grid-item text-container">
            <h4>Julia van der Kris</h4>
            <p>Are you a Full Stack Developer? --PFFFFFFFFF</p>
        </div>

        <div class="grid-item image-container">
            <div class="profile-image jvanmarrewijk"></div>
        </div>
        <div class="grid-item text-container">
            <h4>Jonas van Marrewijk</h4>
            <p>Developer Image Recognition</p>
        </div>
    </div>

</div>


<style lang="scss">
    .meet_the_developers_wrapper {

        h2 {
            text-align: center;
        }

    }

    .grid-container {
        display: grid;
        grid-template-columns: auto auto;
        padding: 1rem;
    }

    .grid-item {
        margin-bottom: 2rem;
        
        h4 {
            margin: 0;
            padding: 0;

            font-family: "Roboto Slab";
            font-weight: 700;
            font-size: 12pt;

            text-align: left;
        }

        p {
            margin: 0;
            padding: 0;
            
            font-size: 12pt;
            
            text-align: left;
        }
    }

    .image-container {
        padding: 1rem;
    }

    .text-container {
        padding-top: 2rem;
        padding-left: 1rem;
    }

    .profile-image {
        width: 100px;
        height: 100px;
        background-size: cover;
        background-position: center center;

        border-radius: 50%;
    }

    .adebruijn {
        background-image: url(/profile-images/adebruijn.jpeg);
        background-position: top center;
    }

    .rdur {
        background-image: url(/profile-images/rdur.jpeg);
    }

    .phengst {
        background-image: url(/profile-images/jvanmarrewijk.jpeg);
    }

    .jvanderkris {
        background-image: url(/profile-images/jvanmarrewijk.jpeg);
    }

    .jvanmarrewijk {
        background-image: url(/profile-images/jvanmarrewijk.jpeg);
        background-position: top center;
    }
</style>